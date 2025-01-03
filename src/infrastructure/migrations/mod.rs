use sea_orm::{DbBackend, Schema};
pub use sea_orm_migration::prelude::*;

mod m20241230_143100_create_user_table;
mod m20241230_143103_create_department_table;
mod m20241230_143102_create_position_table;
mod m20241230_143101_create_organization_table;
mod m20241230_143104_create_employee_department_table;
mod m20241230_143105_create_employee_position_table;
mod m20241230_143106_create_employee_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {

        vec![
            Box::new(m20241230_143100_create_user_table::Migration),
            Box::new(m20241230_143101_create_organization_table::Migration),
            Box::new(m20241230_143103_create_department_table::Migration),
            Box::new(m20241230_143102_create_position_table::Migration),
            Box::new(m20241230_143106_create_employee_table::Migration),
            Box::new(m20241230_143104_create_employee_department_table::Migration),
            Box::new(m20241230_143105_create_employee_position_table::Migration),
        ]
    }
}
