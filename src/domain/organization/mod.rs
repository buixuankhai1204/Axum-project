use crate::domain::user::ERoleUser;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};
use crate::domain::entity::Department;

pub mod request;
pub mod response;
mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "organizations", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm()]
    pub organization_uuid: i64,
    #[sea_orm()]
    pub name: String,
    #[sea_orm()]
    pub description: String,
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
        has_many = "super::department::Entity",
    )]
    Department,
}

// `Related` trait has to be implemented by hand
impl Related<Department> for Entity {
    fn to() -> RelationDef {
        Relation::Department.def()
    }
}
