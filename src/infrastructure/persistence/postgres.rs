use crate::core::configure::AppConfig;
use crate::core::error::AppResult;
use crate::infrastructure::migrations;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;

pub type DatabaseClient = DatabaseConnection;

pub trait DatabaseClientExt: Sized {
    fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = AppResult<Self>>;
}

impl DatabaseClientExt for DatabaseClient {
    async fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        let mut opt = ConnectOptions::new(config.db.get_url());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);
        let db = Database::connect(opt).await?;
        Ok(db)
    }
}

pub async fn migrate_database(db: &DatabaseConnection) -> AppResult {
    tracing::info!("Start migrate database.");
    migrations::Migrator::up(db, None).await?;
    tracing::info!("Migrate database successfully done.");
    Ok(())
}
