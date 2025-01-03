use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::domain::user::EGenderUser;

#[derive(Debug, Serialize, Deserialize, Default, Validate, utoipa::ToSchema)]
pub struct CreateNewEmployeeByUserUuidRequest {
    pub user_uuid: Uuid,
    pub organization_uuid: Uuid,
    pub department_uuid: Uuid,
    pub position_uuid: Uuid,
}

impl CreateNewEmployeeByUserUuidRequest {
    pub fn get_user_uuid(&self) -> Uuid {
        self.user_uuid
    }
    pub fn get_organization_id(&self) -> Uuid {
        self.organization_uuid
    }

    pub fn get_department_uuid(&self) -> Uuid {
        self.department_uuid
    }

    pub fn get_position_uuid(&self) -> Uuid {
        self.position_uuid
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Validate, utoipa::ToSchema)]
pub struct CreateNewEmployeeRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub gender: Option<EGenderUser>,
    pub address: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_two_fa: Option<bool>,
    pub language: Option<String>,
    pub position_uuid: Uuid,
    pub department_uuid: Uuid,
    pub organization_uuid: Uuid,
}

impl CreateNewEmployeeRequest {
    pub fn get_fullname(&self) -> &str {
        &self.fullname
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_gender(&self) -> &Option<EGenderUser> {
        &self.gender
    }
    pub fn get_address(&self) -> &Option<String> {
        &self.address
    }
    pub fn get_role(&self) -> &Option<String> {
        &self.role
    }
    pub fn get_language(&self) -> &Option<String> {
        &self.language
    }
    pub fn get_status_user(&self) -> Option<bool> {
        self.is_active
    }
    pub fn get_status_two_fa(&self) -> Option<bool> {
        self.is_two_fa
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Validate, utoipa::ToSchema)]
pub struct UpdateEmployeeRequest {
    pub employee_uuid: Uuid,
    pub organization_uuid: Uuid,
    pub gender: Option<EGenderUser>,
    pub address: Option<String>,
    pub role: i16,
    pub is_active: Option<bool>,
    pub is_two_fa: Option<bool>,
    pub language: Option<String>,
    pub position_uuid: Option<Vec<Uuid>>,
    pub department_uuid: Option<Vec<Uuid>>,
}

impl UpdateEmployeeRequest {
    pub fn get_employee_uuid(&self) -> Uuid {
        self.employee_uuid
    }
    pub fn get_organization_uuid(&self) -> Uuid {
        self.employee_uuid
    }
    pub fn get_gender(&self) -> &Option<EGenderUser> {
        &self.gender
    }
    pub fn get_address(&self) -> &Option<String> {
        &self.address
    }
    pub fn get_role(&self) -> i16 {
        self.role
    }
    pub fn get_language(&self) -> &Option<String> {
        &self.language
    }
    pub fn get_status_user(&self) -> Option<bool> {
        self.is_active
    }
    pub fn get_status_two_fa(&self) -> Option<bool> {
        self.is_two_fa
    }
    pub fn get_department_uuid(&self) -> Option<Vec<Uuid>> {
        self.department_uuid.clone()
    }
    pub fn get_position_uuid(&self) -> Option<Vec<Uuid>> {
        self.position_uuid.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Validate, utoipa::ToSchema)]
pub struct DeleteEmployeeRequest {
    pub employee_uuid: Uuid,
}

impl DeleteEmployeeRequest {
    pub fn get_employee_uuid(&self) -> Uuid {
        self.employee_uuid
    }
}
