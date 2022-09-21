use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PagedResult<T: Debug> {
  pub items: Vec<T>,
  pub next:  Option<u64>
}
