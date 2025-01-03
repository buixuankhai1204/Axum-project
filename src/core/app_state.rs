use crate::core::configure::AppConfig;
use crate::core::error::AppResult;
use crate::infrastructure::persistence::postgres::{DatabaseClient, DatabaseClientExt};
use crate::infrastructure::persistence::redis_client::instance::{RedisClient, RedisClientBuilder};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
    pub redis: Arc<RedisClient>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let config = Arc::new(config);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let redis = Arc::new(RedisClient::build_from_config(&config)?);

        Ok(Self { config, db, redis })
    }
}
