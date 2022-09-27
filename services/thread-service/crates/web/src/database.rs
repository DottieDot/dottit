use std::{env, sync::Arc};

use database::migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

pub struct Database {
  connection: Arc<DatabaseConnection>
}

impl Database {
  pub async fn connect() -> Self {
    let connection = sea_orm::Database::connect(
      env::var("DATABASE_URL").expect("DATABAE_URL environment variable not set")
    )
    .await
    .expect("could not connect to database");

    Self {
      connection: Arc::new(connection)
    }
  }

  pub fn connection(&self) -> &Arc<DatabaseConnection> {
    &self.connection
  }

  pub async fn migrate(&self) {
    Migrator::up(&self.connection, None)
      .await
      .expect("database migration failed");
  }
}
