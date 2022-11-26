use std::fmt::Debug;

use async_graphql::{OutputType, SimpleObject};
use thread_service_model::models::PagedResult;

use super::Thread;

#[derive(SimpleObject)]
#[graphql(shareable)]
#[graphql(concrete(name = "PagedThreads", params(Thread)))]
pub struct Paged<T: OutputType> {
  next:  Option<u64>,
  items: Vec<T>
}

impl<T: Debug + OutputType> From<PagedResult<T>> for Paged<T> {
  fn from(paged: PagedResult<T>) -> Self {
    Self {
      next:  paged.next,
      items: paged.items
    }
  }
}
