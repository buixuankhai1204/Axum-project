use crate::domain::entity::{EmployeeEntity, PositionEntity, UserEntity};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};

pub mod employee_department;
pub mod employee_position;
pub mod request;
pub mod response;
pub mod services;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    DeriveEntityModel,
    Deserialize,
    Serialize,
)]
#[sea_orm(table_name = "employees", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub employee_uuid: Uuid,
    pub user_id: i64,
    pub image_url: String,
    #[sea_orm(nullable, default_value = true)]
    pub is_active: Option<bool>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::department::Entity> for EmployeeEntity {
    fn to() -> RelationDef {
        employee_department::Relation::Department.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Employee.def().rev())
    }
}

impl Related<PositionEntity> for EmployeeEntity {
    fn to() -> RelationDef {
        employee_position::Relation::Position.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Employee.def().rev())
    }
}

impl Related<UserEntity> for EmployeeEntity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
