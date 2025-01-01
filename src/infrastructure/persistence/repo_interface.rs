use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, ModelTrait};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ReadRepository<E>: Sync + Send
where
    E: EntityTrait + Sync + Send,
    E::Model: Sync + Send,
{
    async fn find_data_by_id(conn: &DatabaseTransaction, id: i64) -> Option<E::Model>;
    async fn find_data_by_uuid(conn: &DatabaseTransaction, uuid: &Uuid) -> Option<E::Model>;
    async fn find_all(conn: &DatabaseTransaction) -> Option<Vec<E::Model>>;

    //base on your business for decision to implement this function with the name you want
    async fn find_data_by_name(conn: &DatabaseTransaction, name: &str) -> Option<E::Model>;
}

#[async_trait::async_trait]
pub trait WriteRepository<E>
where
    E: EntityTrait + Sync + Send,
    E::Model: Sync + Send,
{
    async fn create(conn: &DatabaseTransaction, model: E::Model) -> Option<i64>;

    async fn update(conn: &DatabaseTransaction, model: E::Model) -> Option<i64>;
}

#[async_trait::async_trait]
pub trait DeleteRepository<E>
where
    E: EntityTrait + Sync + Send,
{
    async fn delete(conn: &DatabaseTransaction, id: i64) -> Option<u64>;
}

#[async_trait::async_trait]
pub trait CustomQueryRepository<E>
where
    E: EntityTrait + Sync + Send,
{
    /// Example custom query: find entities by a specific field
    async fn find_by_name(conn: &DatabaseConnection, name: &str) -> Vec<E::Model>;
    async fn find_by_organization_id(
        conn: &DatabaseConnection,
        organization_id: i64,
    ) -> Vec<E::Model>;
}
