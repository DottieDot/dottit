use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use redis::{AsyncCommands, RedisError};
use uuid::Uuid;

use super::traits::{self, DbError};

pub struct AuthTokenDb {
  db: Arc<redis::Client>
}

impl AuthTokenDb {
  pub fn new(db: Arc<redis::Client>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl traits::AuthTokenDb for AuthTokenDb {
  async fn store_token(&self, token: &str, user_id: Uuid) -> Result<(), DbError> {
    let mut conn = self.db.get_async_connection().await?;

    conn.set_nx(token, user_id.to_string()).await?;

    Ok(())
  }

  async fn get_token_owner(&self, token: &str) -> Result<Option<Uuid>, DbError> {
    let mut conn = self.db.get_async_connection().await?;

    if let Some(value) = conn.get::<&str, Option<String>>(token).await? {
      Ok(Some(Uuid::from_str(&value)?))
    } else {
      Ok(None)
    }
  }

  async fn delete_token(&self, token: &str) -> Result<(), DbError> {
    let mut conn = self.db.get_async_connection().await?;

    conn.del(token).await?;

    Ok(())
  }
}

impl From<RedisError> for DbError {
  fn from(error: RedisError) -> Self {
    Self::Unknown {
      source: Box::new(error)
    }
  }
}

impl From<uuid::Error> for DbError {
  fn from(_: uuid::Error) -> Self {
    Self::InvalidUuid
  }
}
