use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::user;
use crate::domain::user::request::{RegisterRequest, UpdateProfileRequest};
use crate::domain::user::response::{GetUserListResponse, GetUserResponse, ProfileResponse};
use crate::infrastructure::persistence::user_repo::{
    repo_check_unique_by_email, repo_check_unique_by_username, repo_find_by_email_and_status,
    repo_find_by_id, repo_find_list, repo_save_user,
};
use crate::util::constant::CODE_LEN;
use crate::util::filter_and_pagination::{EModule, PageQueryParam};
use crate::util::password;
use crate::util::random::generate_random_string;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseTransaction, TransactionTrait};
use tracing::info;

pub async fn service_register_by_email(
    state: AppState,
    req: RegisterRequest,
) -> AppResult<ProfileResponse> {
    info!("Register a new user request: {req:?}.");
    let tx = state.db.begin().await?;
    service_check_unique_username_or_email(&tx, &req.username, &req.email).await?;
    let result = repo_save_user(&tx, req.username, req.password, req.email).await?;
    tx.commit().await?;
    Ok(ProfileResponse::from(result))
}

pub async fn service_get_profile(state: &AppState, user_id: i64) -> AppResult<ProfileResponse> {
    info!("Get user profile with id: {user_id}");
    let result = repo_find_by_id(&*state.db, user_id)
        .await?
        .ok_or_else(|| AppError::EntityNotFoundError { entity: user_id.to_string() })?;
    Ok(ProfileResponse::from(result))
}

pub async fn service_get_list(
    state: &AppState,
    param: PageQueryParam,
) -> AppResult<Vec<GetUserResponse>> {
    info!("Get user list with parameter: {param:?}");
    let results = repo_find_list(&*state.db, param, EModule::User)
        .await?
        .into_iter()
        .map(GetUserResponse::from)
        .collect();
    Ok(results)
}

pub async fn service_update_profile(
    state: &AppState,
    user_id: i64,
    req: UpdateProfileRequest,
) -> AppResult {
    info!("Update user profile with id: {user_id} request: {req:?}");
    let tx = state.db.begin().await?;
    if let Some(username) = req.username.as_ref() {
        repo_check_unique_by_username(&tx, username).await?;
    }
    let mut curr_user: user::ActiveModel = repo_find_by_id(&tx, user_id)
        .await?
        .ok_or_else(|| AppError::EntityNotFoundError { entity: user_id.to_string() })?
        .into();

    if let Some(is_two_fa) = req.is_two_fa {
        curr_user.is_two_fa = Set(Some(is_two_fa));
    }
    if let Some(username) = req.username {
        curr_user.username = Set(username);
    }
    if let Some(password) = req.password {
        curr_user.password = Set(password);
    }
    curr_user.update(&tx).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn service_check_unique_username_or_email(
    tx: &DatabaseTransaction,
    username: &str,
    email: &str,
) -> AppResult {
    repo_check_unique_by_username(tx, username).await?;
    repo_check_unique_by_email(tx, email).await
}
