use crate::core::error::{AppError, AppResult};
use crate::domain::entity::UserEntity;
use crate::domain::model::UserModel;
use crate::domain::user;
use crate::domain::user::ActiveModel;
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use crate::util;
use crate::util::filter_and_pagination::{sort_and_paginate, EModule, PageQueryParam};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction,
    EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use tera::ast::Set;
use uuid::Uuid;

#[async_trait]
impl ReadRepository<UserEntity> for UserEntity {
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<UserModel>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find_by_id(id).one(conn).await {
            Ok(result) => Some(result?),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    async fn find_data_by_uuid<DB>(conn: &DB, uuid: &uuid::Uuid) -> Option<UserModel>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find().filter(user::Column::UserUuid.eq(*uuid)).one(conn).await {
            Ok(result) => Some(result?),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    async fn find_all<DB>(conn: &DB) -> Option<Vec<UserModel>>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find().all(conn).await {
            Ok(results) => Some(results),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    async fn find_data_by_name<DB>(_conn: &DB, _name: &str) -> Option<UserModel>
    where
        DB: ConnectionTrait + Debug,
    {
        todo!()
    }
}

#[async_trait]
impl<'a> WriteRepository<UserEntity> for UserEntity {
    async fn create(conn: &DatabaseTransaction, model: &UserModel) -> Option<i64> {
        match UserEntity::insert(model.clone().into_active_model()).exec(conn).await {
            Ok(result) => Some(result.last_insert_id),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    async fn update(conn: &DatabaseTransaction, model: UserModel) -> Option<i64> {
        match model.into_active_model().save(conn).await {
            Ok(result) => Some(result.id.unwrap()),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }
}

#[async_trait]
impl DeleteRepository<UserEntity> for UserEntity {
    async fn delete_data(conn: &DatabaseTransaction, uuid: Uuid) -> Option<i64> {
        let user = UserEntity::find().filter(user::Column::UserUuid.eq(uuid)).one(conn).await;
        if user.is_err() {
            return None;
        }
        let mut user = user.unwrap().unwrap();

        user.status = 0;
        let user_delete = user.into_active_model().save(conn).await;
        if user_delete.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                user_delete.unwrap_err()
            );
            return None;
        }

        Some(user_delete.unwrap().id.unwrap())
    }
}

impl UserEntity {
    #[tracing::instrument]
    pub async fn repo_find_by_email_and_status<DB>(
        conn: &DB,
        email: &str,
        status: &i16,
    ) -> Option<UserModel>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find()
            .filter(user::Column::Email.eq(email).and(user::Column::Status.eq(*status)))
            .one(conn)
            .await
        {
            Ok(result) => Some(result?),
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    #[tracing::instrument]
    pub async fn repo_check_is_exists_by_email<DB>(conn: &DB, email: &str) -> Option<bool>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find().filter(user::Column::Email.eq(email)).one(conn).await {
            Ok(result) => {
                if result.is_none() {
                    return Some(false);
                }
                Some(true)
            },
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }

    #[tracing::instrument]
    pub async fn repo_check_is_exists_by_phone_number<DB>(
        conn: &DB,
        phone_number: &str,
    ) -> Option<bool>
    where
        DB: ConnectionTrait + Debug,
    {
        match UserEntity::find().filter(user::Column::PhoneNumber.eq(phone_number)).one(conn).await
        {
            Ok(result) => {
                if result.is_none() {
                    return Some(false);
                }
                Some(true)
            },
            Err(err) => {
                tracing::error!("Something happen when query database: {err:#?}");
                None
            },
        }
    }
}
