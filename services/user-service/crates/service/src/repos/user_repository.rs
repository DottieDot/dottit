use async_trait::async_trait;
use model::models::{User, UuidStr};

use super::RepositoryResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn get_user_by_id(&self, id: &UuidStr) -> RepositoryResult<User>;

  async fn create_user(&self, username: String, password_hash: String) -> RepositoryResult<User>;

  async fn delete_user(&self, id: &UuidStr) -> RepositoryResult<()>;
}
