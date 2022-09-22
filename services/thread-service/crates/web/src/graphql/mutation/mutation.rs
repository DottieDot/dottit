use std::sync::Arc;

use async_graphql::Object;
use service::services::traits::{CreateThreadError, ThreadService};

use crate::graphql::query::ThreadQuery;

pub struct Mutation {
  thread_service: Arc<dyn ThreadService>
}

#[Object]
impl Mutation {
  pub async fn create_thread(
    &self,
    board: String,
    user: String,
    title: String,
    text: Option<String>,
    media: Option<String>
  ) -> Result<ThreadQuery, CreateThreadError> {
    self
      .thread_service
      .create_thread(
        &board,
        &user,
        &title,
        match &text {
          Some(str) => Some(str),
          None => None
        },
        match &media {
          Some(str) => Some(str),
          None => None
        }
      )
      .await
      .map(ThreadQuery::from)
  }
}
