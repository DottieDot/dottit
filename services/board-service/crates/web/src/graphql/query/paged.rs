use async_graphql::{OutputType, SimpleObject};
use shared_service::model::Page;

use super::Board;

#[derive(SimpleObject)]
#[graphql(shareable)]
#[graphql(concrete(name = "PagedBoards", params(Board, u64)))]
pub struct Paged<T: OutputType, Key: OutputType> {
  next:  Option<Key>,
  items: Vec<T>,
  total: Option<u64>
}

impl<T: OutputType, K: OutputType> From<Page<T, K>> for Paged<T, K> {
  fn from(paged: Page<T, K>) -> Self {
    Self {
      next:  paged.next,
      items: paged.items,
      total: paged.total
    }
  }
}
