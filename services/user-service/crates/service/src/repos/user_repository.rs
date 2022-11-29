use async_trait::async_trait;
use user_service_model::models::User;
use uuid::Uuid;

use super::RepositoryResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn get_user_by_id(&self, id: Uuid) -> RepositoryResult<User>;

  async fn create_user(&self, username: String, password_hash: String) -> RepositoryResult<User>;

  async fn get_user_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;

  async fn delete_user(&self, id: Uuid) -> RepositoryResult<()>;
}
