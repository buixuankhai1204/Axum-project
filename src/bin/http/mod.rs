use axum::body::Bytes;
use axum::http::{header, HeaderValue};
use axum::{
    middleware::{self},
    Router,
};
use erp_backend::controller::openapi::ApiDoc;
use erp_backend::controller::{build_routes, employee, server, user};
use erp_backend::core::app_state::AppState;
use erp_backend::core::configure::AppConfig;
use erp_backend::core::error::AppResult;
use erp_backend::infrastructure::middleware::map_response::handler_404;
use erp_backend::infrastructure::persistence::postgres::migrate_database;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::ServiceBuilderExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}

impl AppServer {
    pub async fn new(mut config: AppConfig) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        tracing::info!("The server is listening on: {addr}");
        config.server.port = addr.port();
        let state = AppState::new(config).await?;
        migrate_database(&state.db).await?;
        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

        let middleware = ServiceBuilder::new()
            .sensitive_request_headers(sensitive_headers.clone())
            .layer(
                TraceLayer::new_for_http()
                    .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                        tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                    })
                    .make_span_with(
                        DefaultMakeSpan::new().include_headers(true)
                    )
                    .on_response(
                        DefaultOnResponse::new()
                            .include_headers(true)
                            .latency_unit(tower_http::LatencyUnit::Millis)
                    ))
            .sensitive_response_headers(sensitive_headers)
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
            .compression()
            .insert_response_header_if_not_present(
                header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream")
            );

        let app = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(build_routes())
            .layer(CorsLayer::new())
            .layer(middleware)
            .fallback(handler_404)
            .with_state(self.state);

        axum::serve(self.tcp, app).await?;
        Ok(())
    }
}
