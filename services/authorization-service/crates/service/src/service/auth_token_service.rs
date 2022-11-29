use std::sync::Arc;

use async_trait::async_trait;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;

use crate::database::traits::{AuthTokenDb, DbError};

use super::traits;

pub struct AuthTokenService {
  db: Arc<dyn AuthTokenDb>
}

impl AuthTokenService {
  pub fn new(db: Arc<dyn AuthTokenDb>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl traits::AuthTokenService for AuthTokenService {
  async fn create_token_for_user(&self, user_id: Uuid) -> Result<String, DbError> {
    let token = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(64)
      .map(char::from)
      .collect::<String>();

    self.db.store_token(&token, user_id).await?;

    Ok(token)
  }

  async fn get_user_id_from_token(&self, token: &str) -> Result<Option<Uuid>, DbError> {
    self.db.get_token_owner(token).await
  }

  async fn delete_token(&self, token: &str) -> Result<(), DbError> {
    self.db.delete_token(token).await
  }
}
