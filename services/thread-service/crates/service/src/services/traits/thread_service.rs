use model::models::{GuidString, PagedResult, Pagination, Thread};

pub trait ThreadService {
  fn get_thread_by_id(id: GuidString) -> Vec<Thread>;
  fn get_threads_by_board(id: GuidString, pagination: Pagination) -> PagedResult<Thread>;
}
