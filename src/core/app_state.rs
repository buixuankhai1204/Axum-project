use crate::core::configure::AppConfig;
use crate::core::error::AppResult;
use crate::infrastructure::persistence::postgres::{DatabaseClient, DatabaseClientExt};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        Ok(Self { config: Arc::new(config), db })
    }
}
