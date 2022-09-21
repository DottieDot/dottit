use model::models::{GuidString, PagedResult, Pagination, Thread};

use crate::repos::ThreadRepository;

use super::traits;

pub struct ThreadService {
  thread_repo: Box<dyn ThreadRepository>
}

impl traits::ThreadService for ThreadService {
  fn get_thread_by_id(id: GuidString) -> Vec<Thread> {
    todo!()
  }

  fn get_threads_by_board(id: GuidString, pagination: Pagination) -> PagedResult<Thread> {
    todo!()
  }
}
