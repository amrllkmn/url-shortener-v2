pub use sea_orm_migration::prelude::*;

mod m20230816_030414_create_url_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230816_030414_create_url_table::Migration)]
    }
}
