use async_trait::async_trait;
use dyn_clone::DynClone;
use model::models::{PagedResult, Pagination, Thread, UuidStr};

use super::RepositoryResult;

#[async_trait]
pub trait ThreadRepository: Send + Sync + DynClone {
  async fn get_thread_by_id(&self, id: &UuidStr) -> RepositoryResult<Thread>;

  async fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Thread>>;

  async fn create_thread(
    &self,
    board: &str,
    user: &str,
    title: &str,
    text: Option<&str>,
    media: Option<&str>
  ) -> RepositoryResult<Thread>;

  async fn delete_thread(&self, thread_id: &UuidStr) -> RepositoryResult<()>;
}
dyn_clone::clone_trait_object!(ThreadRepository);
