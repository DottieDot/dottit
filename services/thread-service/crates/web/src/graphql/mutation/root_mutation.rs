use std::sync::Arc;

use async_graphql::{Context, Object};
use shared_web::GqlContextExtensions;
use thread_service_service::services::traits::{
  CreateThreadError, DeleteThreadError, ThreadService
};
use uuid::Uuid;

use crate::graphql::query::Thread;

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_thread(
    &self,
    ctx: &Context<'_>,
    board_id: Uuid,
    title: String,
    text: String
  ) -> Result<Thread, CreateThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();
    let user = ctx.authenticated_user().unwrap();

    service
      .create_thread(board_id, user.user_id, title, text)
      .await
      .map(Thread::from)
  }

  pub async fn delete_thread(
    &self,
    ctx: &Context<'_>,
    thread_id: Uuid
  ) -> Result<bool, DeleteThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service.delete_thread(thread_id).await.map(|_| true)
  }
}
