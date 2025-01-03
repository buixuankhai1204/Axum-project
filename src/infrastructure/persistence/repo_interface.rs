use std::cell::RefCell;
use sea_orm::{ConnectionTrait, DatabaseTransaction, EntityTrait};
use std::fmt::Debug;
use std::rc::Rc;
use uuid::Uuid;
use crate::util::filter_and_pagination::PageQueryParam;

#[async_trait::async_trait]
pub trait ReadRepository<E>: Sync + Send
where
    E: EntityTrait + Sync + Send,
    E::Model: Sync + Send,
{
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<E::Model>
    where
        DB: ConnectionTrait + Debug;
    async fn find_data_by_uuid<DB>(conn: &DB, uuid: &Uuid) -> Option<E::Model>
    where
        DB: ConnectionTrait + Debug;
    async fn find_all<DB>(conn: &DB, query_params: PageQueryParam) -> Option<Vec<E::Model>>
    where
        DB: ConnectionTrait + Debug;

    //base on your business for decision to implement this function with the name you want
    async fn find_data_by_name<DB>(conn: &DB, name: &str) -> Option<E::Model>
    where
        DB: ConnectionTrait + Debug;
}

#[async_trait::async_trait]
pub trait WriteRepository<E>
where
    E: EntityTrait + Sync + Send,
    E::Model: Sync + Send,
{
    async fn create(conn: &DatabaseTransaction, model: &E::Model) -> Option<i64>;

    async fn update(conn: &DatabaseTransaction, model: E::Model) -> Option<i64>;
}

#[async_trait::async_trait]
pub trait DeleteRepository<E>
where
    E: EntityTrait + Sync + Send,
{
    async fn delete_data(conn: &DatabaseTransaction, id: Uuid) -> Option<i64>;
}
