pub use sea_orm_migration::prelude::*;

mod m20241216_114200_create_user_table;
mod m20241218_095500_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241216_114200_create_user_table::Migration),
            Box::new(m20241218_095500_create_user_table::Migration),
        ]
    }
}
