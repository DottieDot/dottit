use std::fmt::Debug;

use async_graphql::{OutputType, SimpleObject};
use chrono::{DateTime, Utc};
use shared_service::model::Page;

use super::Comment;

type DtUtc = DateTime<Utc>;

#[derive(SimpleObject)]
#[graphql(shareable)]
#[graphql(concrete(name = "PagedComments", params(Comment, DtUtc)))]
pub struct Paged<T: OutputType, Key: OutputType> {
  next:  Option<Key>,
  items: Vec<T>
}

impl<T: Debug + OutputType> From<Page<T, DateTime<Utc>>> for Paged<T, DateTime<Utc>> {
  fn from(paged: Page<T, DateTime<Utc>>) -> Self {
    Self {
      next:  paged.next,
      items: paged.items
    }
  }
}
