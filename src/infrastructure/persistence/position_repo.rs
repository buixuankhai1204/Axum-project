use crate::core::error::{AppError, AppResult};
use crate::domain::entity::PositionEntity;
use crate::domain::model::PositionModel;
use crate::domain::position::Column;
use crate::domain::{employee, position};
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use std::cell::RefCell;
use uuid::Uuid;
use crate::util::filter_and_pagination::{sort_and_paginate, EModule, PageQueryParam};

#[async_trait]
impl ReadRepository<PositionEntity> for PositionEntity {
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<PositionModel>
    where
        DB: ConnectionTrait,
    {
        let user = PositionEntity::find_by_id(id).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_data_by_uuid<DB>(conn: &DB, uuid: &uuid::Uuid) -> Option<PositionModel>
    where
        DB: ConnectionTrait,
    {
        let user = PositionEntity::find().filter(Column::PositionUuid.eq(*uuid)).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_all<DB>(conn: &DB, query_params: PageQueryParam) -> Option<Vec<PositionModel>>
    where
        DB: ConnectionTrait,
    {
        let positions = sort_and_paginate(
            conn,
            &mut PositionEntity::find(),
            query_params,
            EModule::Position,
        ).await;
        if positions.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", positions.unwrap_err());
            return None;
        };
        Some(positions.unwrap_or_default())
    }

    async fn find_data_by_name<DB>(conn: &DB, name: &str) -> Option<PositionModel>
    where
        DB: ConnectionTrait,
    {
        let user = PositionEntity::find().filter(Column::Name.eq(name)).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }
}

// #[async_trait]
// impl WriteRepository<PositionEntity> for PositionEntity {
//     async fn create(
//         conn: &DatabaseTransaction,
//         model: & PositionModel,
//     ) -> Option<i64> {
//         let position = PositionEntity::insert(model.clone().into_active_model()).exec(conn).await;
//         if position.is_err() {
//             tracing::error!("Something happen when query database: {:#?}.", position.unwrap_err());
//             return None;
//         };
//         Some(position.unwrap().last_insert_id)
//     }
//
//     async fn update(conn: &DatabaseTransaction, model: PositionModel) -> Option<i64> {
//         let user_update = model.into_active_model().save(conn).await;
//         if user_update.is_err() {
//             tracing::error!(
//                 "Something happen when query database: {:#?}.",
//                 user_update.unwrap_err()
//             );
//             return None;
//         };
//
//         Some(user_update.unwrap().id.unwrap())
//     }
// }

#[async_trait]
impl DeleteRepository<PositionEntity> for PositionEntity {
    async fn delete_data(conn: &DatabaseTransaction, uuid: Uuid) -> Option<i64> {
        let position =
            PositionEntity::find().filter(position::Column::PositionUuid.eq(uuid)).one(conn).await;
        if position.is_err() {
            return None;
        }
        let mut position = position.unwrap().unwrap();

        position.status = Some(0);
        let position_delete = position.into_active_model().save(conn).await;
        if position_delete.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                position_delete.unwrap_err()
            );
            return None;
        }

        Some(position_delete.unwrap().id.unwrap())
    }
}

impl PositionEntity {}
