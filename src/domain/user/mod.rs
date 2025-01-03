use crate::domain::entity::{EmployeeEntity, UserEntity};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, EnumIter};
use serde::{Deserialize, Serialize};

pub mod request;
pub mod response;
pub mod services;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,Default
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique, indexed)]
    pub user_uuid: Uuid,
    #[sea_orm()]
    pub role_id: i64,
    #[sea_orm()]
    pub creator_id: i64,
    #[sea_orm(unique, indexed)]
    pub email: String,
    pub password: String,
    pub full_name: String,
    #[sea_orm(default_value = "Other")]
    pub gender: Option<EGenderUser>,
    #[sea_orm(unique, indexed)]
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub picture: Option<String>,
    #[sea_orm(default_value = "vi")]
    pub language: Option<String>,
    #[sea_orm(default_value = 1)]
    pub status: i16,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
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
    Default,
)]
#[sea_orm(rs_type = "String", db_type = "Text", enum_name = "GENDER_OF_USER")]
pub enum EGenderUser {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
    #[sea_orm(string_value = "Other")]
    #[default]
    Other,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::employee::Entity")]
    Employee,
}

// `Related` trait has to be implemented by hand
impl Related<EmployeeEntity> for UserEntity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
