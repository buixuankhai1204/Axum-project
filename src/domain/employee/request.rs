use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Validate, utoipa::ToSchema)]
pub struct CreateNewEmployeeByUserUuidRequest {
    #[garde(skip)]
    pub user_uuid: Uuid,
    #[garde(skip)]
    pub organization_uuid: Uuid,
    #[garde(skip)]
    pub department_uuid: Uuid,
    #[garde(skip)]
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
