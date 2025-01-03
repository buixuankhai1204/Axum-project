use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel, Deserialize, Serialize,
)]
#[sea_orm(table_name = "employee_position", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub employee_id: i64,
    #[sea_orm(primary_key)]
    pub position_id: i64,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::EmployeeId",
        to = "crate::domain::employee::Column::Id"
    )]
    Employee,
    #[sea_orm(
        belongs_to = "crate::domain::position::Entity",
        from = "Column::PositionId",
        to = "crate::domain::position::Column::Id"
    )]
    Position,
}
