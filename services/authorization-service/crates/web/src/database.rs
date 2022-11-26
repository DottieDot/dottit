use std::{env, sync::Arc};

use redis::Client;

pub struct Database {
  connection: Arc<Client>
}

impl Database {
  pub fn connect() -> Self {
    let connection =
      Client::open(env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"))
        .expect("could not connect to database");

    Self {
      connection: Arc::new(connection)
    }
  }

  pub fn connection(&self) -> &Arc<Client> {
    &self.connection
  }
}
