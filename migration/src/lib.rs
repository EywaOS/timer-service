pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20241230_000001_create_tsaheylu_schema;
mod m20260106_000001_create_audit_log;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241230_000001_create_tsaheylu_schema::Migration),
            Box::new(m20260106_000001_create_audit_log::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        SeaRc::new(Alias::new("tsaheylu_migrations"))
    }
}
