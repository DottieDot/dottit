use std::error::Error;

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[async_trait]
pub trait AuthTokenDb: Send + Sync {
  async fn store_token(&self, token: &str, user_id: Uuid) -> Result<(), DbError>;

  async fn get_token_owner(&self, token: &str) -> Result<Option<Uuid>, DbError>;

  async fn delete_token(&self, token: &str) -> Result<(), DbError>;
}

#[derive(Error, Debug)]
pub enum DbError {
  #[error("An invalid uuid was stored")]
  InvalidUuid,
  #[error("An unknown database error has occurred: {source}")]
  Unknown {
    source: Box<dyn Error + Send + Sync>
  }
}
