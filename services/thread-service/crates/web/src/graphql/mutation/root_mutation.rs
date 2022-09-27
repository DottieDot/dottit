use std::sync::Arc;

use async_graphql::{Context, Object};
use service::services::traits::{CreateThreadError, ThreadService};

use crate::graphql::query::ThreadQuery;

pub struct RootMutation;

#[Object]
impl RootMutation {
  pub async fn create_thread(
    &self,
    ctx: &Context<'_>,
    board: String,
    user: String,
    title: String,
    text: Option<String>,
    media: Option<String>
  ) -> Result<ThreadQuery, CreateThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service
      .create_thread(&board, &user, &title, text.as_deref(), media.as_deref())
      .await
      .map(ThreadQuery::from)
  }
}