use crate::domain::employee::{employee_department, employee_position};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};
use crate::domain::entity::{Employee, Organization, Position};

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
    pub department_uuid: i64,
    #[sea_orm()]
    pub organization_id: i64,
    #[sea_orm(primary_key)]
    pub image_url: String,
    #[sea_orm(primary_key)]
    pub is_active: bool,
    #[sea_orm(primary_key)]
    pub create_at: DateTime<Utc>,
    #[sea_orm(primary_key)]
    pub update_at: DateTime<Utc>,
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
    #[sea_orm(
    has_many = "super::position::Entity"
    )]
    Position,
}

impl Related<Organization> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<Position> for Entity {
    fn to() -> RelationDef {
        Relation::Position.def()
    }
}

impl Related<Employee> for Entity {
    fn to() -> RelationDef {
        employee_department::Relation::Employee.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Department.def().rev())
    }
}

