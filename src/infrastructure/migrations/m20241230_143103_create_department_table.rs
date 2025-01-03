use crate::domain::user;
use crate::infrastructure::persistence::postgres::{DatabaseClient, DatabaseClientExt};
use sea_orm::{DbBackend, Schema};
use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};
use std::sync::Arc;
use crate::domain::entity::DepartmentEntity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let schema = Schema::new(db_postgres);
        let db = manager.get_connection();
        let statement = db_postgres.build(&schema.create_table_from_entity(DepartmentEntity));
        println!("Creating table departments {}", statement.sql.as_str());
        db.execute_unprepared(statement.sql.as_str()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE IF EXISTS departments").await?;
        Ok(())
    }
}
