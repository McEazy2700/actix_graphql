pub use sea_orm_migration::prelude::*;

mod m20230415_212103_create_user_and_profile_tables;
pub mod table_enums;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230415_212103_create_user_and_profile_tables::Migration),
        ]
    }
}
