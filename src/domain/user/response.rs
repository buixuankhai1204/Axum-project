use crate::domain::user;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetUserListResponse {
    pub results: Vec<GetUserResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetUserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role_name: String,
    pub is_active: Option<bool>,
    pub is_two_fa: Option<bool>,
    pub create_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total: i64,
}

impl<T> PageResponse<T> {
    pub fn new(data: Vec<T>, page_num: i64, page_size: i64, total: i64) -> PageResponse<T> {
        PageResponse { data, page_num, page_size, total }
    }

    pub fn map<F, B>(&self, f: F) -> PageResponse<B>
    where
        F: FnMut(&T) -> B,
    {
        let data: Vec<B> = self.data.iter().map(f).collect();
        PageResponse { data, page_num: self.page_num, page_size: self.page_size, total: self.total }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfileResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub is_active: Option<bool>,
    pub is_two_fa: Option<bool>,
    pub create_at: NaiveDateTime,
}

impl From<user::Model> for ProfileResponse {
    fn from(user: user::Model) -> Self {
        ProfileResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            is_two_fa: user.is_two_fa,
            create_at: user.create_at,
        }
    }
}

impl From<user::Model> for GetUserResponse {
    fn from(user: user::Model) -> Self {
        GetUserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role_name: user.role.to_string(),
            is_active: user.is_active,
            is_two_fa: user.is_two_fa,
            create_at: user.create_at,
        }
    }
}
