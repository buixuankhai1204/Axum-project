use crate::http::AppServer;
use erp_backend::core::configure;
use erp_backend::core::error::AppResult;
use erp_backend::util::constant::CONFIG;
use std::sync::Arc;

mod http;

#[tokio::main]
async fn main() -> AppResult<()> {
    let _file_appender_guard = configure::trace::init()?;
    tracing::info!("The initialization of Tracing was successful!");
    let config = CONFIG.clone();
    tracing::info!("Reading the config file was successful!");
    tracing::info!("Create a new server!");
    // let _sentry_guard = configure::sentry::init(&config);
    let server = AppServer::new(config).await?;
    tracing::info!("Run the server!");
    server.run().await?;
    Ok(())
}
