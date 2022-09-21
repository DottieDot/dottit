use model::models::{GuidStr, PagedResult, Pagination, Thread};

use super::RepositoryResult;

pub trait ThreadRepository {
  fn get_thread_by_id(&self, id: &GuidStr) -> RepositoryResult<Thread>;

  fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Thread>>;

  fn create_thread<'a>(
    &self,
    board: &str,
    user: &str,
    text: &str,
    media: &str
  ) -> RepositoryResult<Thread>;
}
