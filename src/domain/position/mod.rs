use crate::domain::employee::employee_position;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};
use crate::domain::entity::{DepartmentEntity, EmployeeEntity, PositionEntity};

pub mod request;
pub mod response;
mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "positions", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm()]
    pub position_uuid: i64,
    #[sea_orm()]
    pub department_id: i64,
    #[sea_orm(indexd, unique)]
    pub name: String,
    #[sea_orm(nulable)]
    pub description: String,
    #[sea_orm(nullable, default_value = true)]
    pub is_active: Option<bool>,
    #[sea_orm()]
    pub create_at: DateTime<Utc>,
    #[sea_orm()]
    pub update_at: DateTime<Utc>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::department::Entity",
        from = "Column::DepartmentId",
        to = "super::department::Column::Id"
    )]
    Department,
}

// `Related` trait has to be implemented by hand
impl Related<DepartmentEntity> for PositionEntity {
    fn to() -> RelationDef {
        Relation::Department.def()
    }
}

impl Related<EmployeeEntity> for PositionEntity {
    fn to() -> RelationDef {
        employee_position::Relation::Position.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_position::Relation::Employee.def().rev())
    }
}
