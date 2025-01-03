use crate::domain::user;
use crate::domain::user::EGenderUser;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicProfileResponse {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub gender: Option<EGenderUser>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub language: Option<String>,
    pub status: Option<i16>,
    pub role_id: i64,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<user::Model> for PublicProfileResponse {
    fn from(user: user::Model) -> Self {
        PublicProfileResponse {
            user_id: user.user_uuid,
            email: user.email,
            full_name: user.full_name,
            gender: user.gender,
            phone_number: user.phone_number,
            address: user.address,
            language: user.language,
            status: Option::from(user.status),
            role_id: user.role_id,
            last_login: user.last_login,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}
