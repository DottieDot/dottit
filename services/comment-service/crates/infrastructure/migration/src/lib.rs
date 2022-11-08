pub use sea_orm_migration::prelude::*;

mod m20220921_233556_create_threads_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20220921_233556_create_threads_table::Migration)]
  }
}
