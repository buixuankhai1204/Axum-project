use crate::domain::user::ERoleUser;
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};
use crate::domain::entity::{Employee, Position, User};

pub mod employee_department;
pub mod employee_position;
pub mod request;
pub mod response;
pub mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "employees", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub employee_uuid: Uuid,
    pub user_id: i64,
    pub image_url: String,
    pub role: ERoleUser,
    pub is_active: bool,
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

impl Related<super::department::Entity> for Employee {
    fn to() -> RelationDef {
        employee_department::Relation::Department.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Employee.def().rev())
    }
}

impl Related<Position> for Employee {
    fn to() -> RelationDef {
        employee_position::Relation::Position.def()
    }

    fn via() -> Option<RelationDef> {
        Some(employee_department::Relation::Employee.def().rev())
    }
}

impl Related<User> for Employee {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
