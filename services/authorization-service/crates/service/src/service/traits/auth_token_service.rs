use async_trait::async_trait;

use crate::database::traits::DbError;

#[async_trait]
pub trait AuthTokenService {
  async fn create_token_for_user(&self, user_id: &str) -> Result<String, DbError>;

  async fn get_user_id_from_token(&self, token: &str) -> Result<Option<String>, DbError>;

  async fn delete_token(&self, token: &str) -> Result<(), DbError>;
}
