pub use sea_orm_migration::prelude::*;

mod m20250630_000001_create_user_and_role;
mod m20250630_000002_add_user_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250630_000001_create_user_and_role::Migration),
            Box::new(m20250630_000002_add_user_fields::Migration),
        ]
    }
}
