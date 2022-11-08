use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use service::services::traits::{CreateThreadError, DeleteThreadError, ThreadService};

use crate::graphql::query::Thread;

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_thread(
    &self,
    ctx: &Context<'_>,
    board: String,
    user: String,
    title: String,
    text: Option<String>,
    media: Option<String>
  ) -> Result<Thread, CreateThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service
      .create_thread(&board, &user, &title, text.as_deref(), media.as_deref())
      .await
      .map(Thread::from)
  }

  pub async fn delete_thread(
    &self,
    ctx: &Context<'_>,
    thread_id: ID
  ) -> Result<bool, DeleteThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service.delete_thread(&thread_id).await.map(|_| true)
  }
}
