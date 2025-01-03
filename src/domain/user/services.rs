use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::user;
use crate::domain::user::request::AdminCreateAccountRequest;
use crate::domain::user::request::UpdateProfileRequest;
use crate::domain::user::response::PublicProfileResponse;
use crate::domain::user::{EGenderUser, UserEntity};
use crate::infrastructure::persistence::repo_interface::{ReadRepository, WriteRepository};
use crate::util::constant::CODE_LEN;
use crate::util::filter_and_pagination::{EModule, PageQueryParam};
use crate::util::password;
use crate::util::random::generate_random_string;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use sea_orm::{DatabaseTransaction, TransactionTrait};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub async fn service_get_profile(
    state: &AppState,
    user_uid: Uuid,
) -> AppResult<PublicProfileResponse> {
    tracing::info!("Get user profile with uuid: {user_uid}");
    match UserEntity::find_data_by_uuid(&*state.db, &user_uid).await {
        Some(profile) => Ok(PublicProfileResponse::from(profile)),
        None => Err(AppError::EntityNotFoundError {
            detail: format!("User not found by id {}", user_uid),
        }),
    }
}

pub async fn service_update_profile(
    state: &AppState,
    user_uid: Uuid,
    req: UpdateProfileRequest,
) -> AppResult {
    tracing::info!("Update user profile with id: {user_uid} request: {req:?}");
    let tx = state.db.begin().await?;
    if let Some(phone_number) = req.phone_number.as_ref() {
        service_check_is_exists_by_phone_number(&tx, phone_number).await?;
    }

    let mut curr_user = UserEntity::find_data_by_uuid(&tx, &user_uid).await.ok_or_else(|| {
        AppError::EntityNotFoundError { detail: format!("User not found by id {}", user_uid) }
    })?;

    if let Some(full_name) = req.full_name {
        curr_user.full_name = full_name;
    }
    if let Some(gender) = req.gender {
        curr_user.gender = Some(gender.parse()?);
    }
    if let Some(phone_number) = req.phone_number {
        curr_user.phone_number = Some(phone_number);
    }
    if let Some(address) = req.address {
        curr_user.address = Some(address);
    }
    if let Some(language) = req.language {
        curr_user.language = Some(language);
    }
    if let Some(status) = req.status {
        curr_user.status = status;
    }
    curr_user.into_active_model().update(&tx).await?;
    tx.commit().await?;
    Ok(())
}

// Administrator

pub async fn service_admin_create_account(
    state: &AppState,
    req: AdminCreateAccountRequest,
) -> AppResult<PublicProfileResponse> {
    tracing::info!("Admin create a new account request: {req:?}.");
    let tx = state.db.begin().await?;
    let is_exists_email = UserEntity::repo_check_is_exists_by_email(&tx, &req.email).await;
    if is_exists_email.is_none() {
        return Err(AppError::BadRequestError("Something went wrong!".to_string()));
    }
    if is_exists_email.unwrap() {
        return Err(AppError::EntityExistsError { detail: "Email is already exists!".to_string() });
    }

    if let Some(phone_number) = req.phone_number.as_ref() {
        service_check_is_exists_by_phone_number(&tx, phone_number).await?;
    }

    let hash_password = password::hash((&req.password).to_string()).await?;
    let mut user_model = user::Model {
        id: Default::default(),
        user_uuid: Uuid::new_v4(),
        full_name: req.full_name,
        picture: req.picture,
        gender: Some(EGenderUser::Other), // TODO: build matching gender with req.gender here
        phone_number: req.phone_number,
        address: req.address,
        language: Some(req.language.unwrap_or("vi".to_string())),
        status: req.status.unwrap_or(1),
        role_id: 222,    // TODO: find role id by req.role_id and add id here
        creator_id: 321, // TODO: add creator id by jwt
        password: hash_password,
        email: (&req.email).to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        last_login: Default::default(),
        deleted_at: Default::default(),
    };

    let data = UserEntity::create(&tx, &user_model).await;
    tx.commit().await?;
    Ok(PublicProfileResponse::from(user_model))
}

pub async fn service_admin_get_list(
    state: &AppState,
    param: PageQueryParam,
) -> AppResult<Vec<PublicProfileResponse>> {
    tracing::info!("Get user list with parameter: {param:?}");
    match UserEntity::find_all(&*state.db).await {
        Some(result) => Ok(result.into_iter().map(PublicProfileResponse::from).collect()),
        None => Err(AppError::BadRequestError("Something went wrong!".to_string())),
    }
}

// TODO: implement service admin update profile with role id and create new employee

async fn service_check_is_exists_by_phone_number(
    tx: &DatabaseTransaction,
    phone_number: &str,
) -> AppResult {
    let is_exists_phone_number =
        UserEntity::repo_check_is_exists_by_phone_number(tx, phone_number).await;

    if is_exists_phone_number.is_none() {
        return Err(AppError::BadRequestError("Something went wrong!".to_string()));
    }

    if is_exists_phone_number.unwrap() {
        return Err(AppError::EntityExistsError {
            detail: "Phone number is already exists!".to_string(),
        });
    }

    Ok(())
}
