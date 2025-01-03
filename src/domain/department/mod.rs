use crate::domain::employee::{employee_department, employee_position};
use crate::domain::entity::{DepartmentEntity, EmployeeEntity, OrganizationEntity, PositionEntity};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};

pub mod request;
pub mod response;
mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "departments", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm()]
    pub department_uuid: Uuid,
    #[sea_orm()]
    pub organization_id: i64,
    #[sea_orm()]
    pub name: String,
    #[sea_orm()]
    pub image_url: String,
    #[sea_orm(nullable, default_value = 1)]
    pub status: Option<i16>,
    #[sea_orm()]
    pub create_at: NaiveDateTime,
    #[sea_orm()]
    pub update_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::organization::Entity"
    from = "Column::OrganizationId",
    to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(has_many = "super::position::Entity")]
    Position,
}

impl Related<OrganizationEntity> for DepartmentEntity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<PositionEntity> for DepartmentEntity {
    fn to() -> RelationDef {
        Relation::Position.def()
    }
}

impl Related<EmployeeEntity> for DepartmentEntity {
    fn to() -> RelationDef {
        employee_department::Relation::Employee.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Department.def().rev())
    }
}
