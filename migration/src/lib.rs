pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20241230_000001_create_tsaheylu_schema;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241230_000001_create_tsaheylu_schema::Migration)]
    }

    fn migration_table_name() -> DynIden {
        SeaRc::new(Alias::new("tsaheylu_migrations"))
    }
}
