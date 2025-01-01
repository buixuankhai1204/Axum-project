use crate::core::error::{AppError, AppResult};
use crate::domain::entity::User;
use crate::domain::user::Entity;
use crate::domain::{organization, user};
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use crate::util;
use crate::util::filter_and_pagination::{sort_and_paginate, EModule, PageQueryParam};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, IntoActiveModel, QueryFilter, Set,
};

#[tracing::instrument]
pub async fn repo_save_user(
    tx: &DatabaseTransaction,
    username: String,
    password: String,
    email: String,
) -> AppResult<user::Model> {
    let hash_password = util::password::hash(password).await?;
    let user = user::ActiveModel {
        username: Set(username),
        password: Set(hash_password),
        email: Set(email),
        create_at: Set(Utc::now().naive_utc()),
        update_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(tx)
    .await?;
    Ok(user)
}

#[tracing::instrument(skip_all)]
pub async fn repo_find_by_id<DB>(db: &DB, id: i64) -> AppResult<Option<user::Model>>
where
    DB: ConnectionTrait,
{
    let model = user::Entity::find_by_id(id).one(db).await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn repo_find_list(
    db: &DatabaseConnection,
    param: PageQueryParam,
    module_name: EModule,
) -> AppResult<Vec<user::Model>> {
    let mut select = user::Entity::find();
    let results = sort_and_paginate(db, &mut select, param, module_name).await;
    match results {
        Ok(value) => Ok(value),
        Err(error) => Err(error),
    }
}

#[tracing::instrument(skip_all)]
pub async fn repo_find_by_email_and_status(
    db: &DatabaseConnection,
    email: &str,
    is_active: bool,
) -> AppResult<Option<user::Model>> {
    let query = user::Column::Email.eq(email).and(user::Column::IsActive.eq(is_active));
    let user = user::Entity::find().filter(query).one(db).await?;
    Ok(user)
}

#[tracing::instrument]
pub async fn repo_check_unique_by_email(tx: &DatabaseTransaction, email: &str) -> AppResult<()> {
    let result = user::Entity::find().filter(user::Column::Email.eq(email)).one(tx).await?;
    if result.is_some() {
        Err(AppError::EntityExistsError { entity: email.to_string() })
    } else {
        Ok(())
    }
}

#[tracing::instrument]
pub async fn repo_check_unique_by_username(tx: &DatabaseTransaction, username: &str) -> AppResult {
    let result = user::Entity::find().filter(user::Column::Username.eq(username)).one(tx).await?;
    if result.is_some() {
        Err(AppError::EntityExistsError { entity: username.to_string() })
    } else {
        Ok(())
    }
}

#[async_trait]
impl ReadRepository<Entity> for Entity {
    async fn find_data_by_id(conn: &DatabaseTransaction, id: i64) -> Option<user::Model> {
        let user = Entity::find_by_id(id).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_data_by_uuid(
        conn: &DatabaseTransaction,
        uuid: &uuid::Uuid,
    ) -> Option<user::Model> {
        let user = Entity::find().filter(user::Column::UserUuid.eq(*uuid)).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_all(conn: &DatabaseTransaction) -> Option<Vec<user::Model>> {
        let users = Entity::find().all(conn).await;
        if users.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", users.unwrap_err());
            return None;
        };
        Some(users.unwrap_or_default())
    }

    async fn find_data_by_name(conn: &DatabaseTransaction, name: &str) -> Option<user::Model> {
        let user = Entity::find().filter(user::Column::Username.eq(name)).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }
}

#[async_trait]
impl WriteRepository<User> for User {
    async fn create(conn: &DatabaseTransaction, model: user::Model) -> Option<i64> {
        let user = User::insert(model.into_active_model()).exec(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        Some(user.unwrap().last_insert_id)
    }

    async fn update(conn: &DatabaseTransaction, model: user::Model) -> Option<i64> {
        let user_update = model.into_active_model().save(conn).await;
        if user_update.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                user_update.unwrap_err()
            );
            return None;
        };

        Some(user_update.unwrap().id.unwrap())
    }
}

#[async_trait]
impl DeleteRepository<User> for User {
    async fn delete(conn: &DatabaseTransaction, id: i64) -> Option<u64> {
        let user_delete = User::delete_by_id(id).exec(conn).await;
        if user_delete.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                user_delete.unwrap_err()
            );
            return None;
        }

        Some(user_delete.unwrap().rows_affected)
    }
}

impl User {
    #[tracing::instrument]
    pub async fn repo_check_unique_by_email(
        tx: &DatabaseTransaction,
        email: &str,
    ) -> AppResult<()> {
        let result = user::Entity::find().filter(user::Column::Email.eq(email)).one(tx).await?;
        if result.is_some() {
            Err(AppError::EntityExistsError { entity: email.to_string() })
        } else {
            Ok(())
        }
    }
}
