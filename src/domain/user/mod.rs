use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, EnumIter};
use serde::{Deserialize, Serialize};
use crate::domain::entity::Employee;

pub mod request;
pub mod response;
pub mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_uuid: Uuid,
    pub fullname: String,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique, nullable, indexed)]
    pub password: String,
    #[sea_orm(unique, nullable, indexed)]
    pub email: String,
    #[sea_orm(default_value = "Member")]
    pub role: ERoleUser,
    #[sea_orm(nullable, default_value = "Male")]
    pub gender: EGender,
    #[sea_orm(nullable)]
    pub address: String,
    #[sea_orm(nullable, default_value = true)]
    pub is_active: Option<bool>,
    #[sea_orm(nullable, default_value = false)]
    pub is_two_fa: Option<bool>,
    #[sea_orm(nullable)]
    pub create_at: NaiveDateTime,
    #[sea_orm(nullable)]
    pub update_at: NaiveDateTime,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    strum::EnumString,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    utoipa::ToSchema,
    Clone,
    Copy,
    EnumIter,
    strum::Display,
    Hash,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Text", enum_name = "ROLE_USER")]
pub enum ERoleUser {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Member")]
    Member,
    #[sea_orm(string_value = "User")]
    User,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    strum::EnumString,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    utoipa::ToSchema,
    Clone,
    Copy,
    EnumIter,
    strum::Display,
    Hash,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Text", enum_name = "E_GENDER")]
pub enum EGender {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::employee::Entity")]
    Employee,
}

// `Related` trait has to be implemented by hand
impl Related<Employee> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
