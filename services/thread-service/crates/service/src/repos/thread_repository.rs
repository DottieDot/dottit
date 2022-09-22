use async_trait::async_trait;
use dyn_clone::DynClone;
use model::models::{GuidStr, PagedResult, Pagination, Thread};

use super::RepositoryResult;

#[async_trait]
pub trait ThreadRepository: Send + Sync + DynClone {
  async fn get_thread_by_id(&self, id: &GuidStr) -> RepositoryResult<Thread>;

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

  async fn delete_thread(&self, thread_id: &GuidStr) -> RepositoryResult<()>;
}
dyn_clone::clone_trait_object!(ThreadRepository);
