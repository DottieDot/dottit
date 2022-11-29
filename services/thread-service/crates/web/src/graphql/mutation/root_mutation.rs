use std::sync::Arc;

use async_graphql::{Context, Object, SimpleObject, Union};
use shared_web::GqlContextExtensions;
use thread_service_model::models;
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
  ) -> Result<CreateThreadErrorsResponse, CreateThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();
    if let Some(user) = ctx.authenticated_user() {
      service
        .create_thread(board_id, user.user_id, title, text)
        .await
        .map(CreateThreadErrorsResponse::from)
    } else {
      Ok(CreateThreadErrorsResponse::Unauthorized(Unauthorized {
        message: "Unauthorized!".to_owned()
      }))
    }
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

#[derive(SimpleObject)]
pub struct Unauthorized {
  message: String
}

#[derive(Union)]
pub enum CreateThreadErrorsResponse {
  Unauthorized(Unauthorized),
  Created(Thread)
}

impl From<models::Thread> for CreateThreadErrorsResponse {
  fn from(thread: models::Thread) -> Self {
    Self::Created(thread.into())
  }
}
