use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dyn_clone::DynClone;
use shared_service::model::{Page, Pagination};
use thread_service_model::models::Thread;
use uuid::Uuid;

use super::RepositoryResult;

#[async_trait]
pub trait ThreadRepository: Send + Sync + DynClone {
  async fn get_thread_by_id(&self, id: Uuid) -> RepositoryResult<Thread>;

  async fn get_threads_by_board(
    &self,
    board: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> RepositoryResult<Page<Thread, DateTime<Utc>>>;

  async fn get_threads_by_user(
    &self,
    user: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> RepositoryResult<Page<Thread, DateTime<Utc>>>;

  async fn create_thread(
    &self,
    board_id: Uuid,
    user_id: Uuid,
    title: String,
    text: String
  ) -> RepositoryResult<Thread>;

  async fn delete_thread(&self, thread_id: Uuid) -> RepositoryResult<()>;
}
dyn_clone::clone_trait_object!(ThreadRepository);
