pub use sea_orm_migration::prelude::*;

mod m20240801_000001_create_ingestion_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>>
    {
        vec![Box::new(m20240801_000001_create_ingestion_table::Migration)]
    }
}
