use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminCreateAccountRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 2, max = 30))]
    pub full_name: String,
    #[validate(url)]
    pub picture: Option<String>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub language: Option<String>,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i16>,
    pub role_id: Uuid,
    pub is_create_new_employee: bool,
}

impl AdminCreateAccountRequest {
    pub fn new(
        email: String,
        password: String,
        full_name: String,
        picture: Option<String>,
        gender: Option<String>,
        phone_number: Option<String>,
        address: Option<String>,
        language: Option<String>,
        status: Option<i16>,
        role_id: Uuid,
        is_create_new_employee: bool,
    ) -> Self {
        Self {
            email,
            password,
            full_name,
            picture,
            gender,
            phone_number,
            address,
            language,
            status,
            role_id,
            is_create_new_employee,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

// TODO: Implement admin update profile request
// #[validate(skip)]
// pub role_id: Option<Uuid>,

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 30))]
    pub full_name: Option<String>,
    #[validate(url)]
    pub picture: Option<String>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub language: Option<String>,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i16>,
}
