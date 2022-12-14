use std::sync::Arc;

use async_graphql::{Context, Object, Union};
use shared_web::{
  gql::{Unauthenticated, ValidationError},
  GqlContextExtensions
};
use thread_service_service::{
  model::dtos::CreateThreadDto,
  services::traits::{CreateThreadError, DeleteThreadError, ThreadService}
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
  ) -> Result<CreateThreadResult, CreateThreadError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();
    if let Some(user) = ctx.authenticated_user() {
      let thread = service
        .create_thread(CreateThreadDto {
          user_id: user.user_id,
          board_id,
          title,
          text
        })
        .await?;

      match thread {
        Ok(thread) => Ok(CreateThreadResult::Created(thread.into())),
        Err(e) => Ok(CreateThreadResult::ValidationError(e.into()))
      }
    } else {
      Ok(CreateThreadResult::Unauthenticated(
        Unauthenticated::default()
      ))
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

#[derive(Union)]
pub enum CreateThreadResult {
  Unauthenticated(Unauthenticated),
  ValidationError(ValidationError),
  Created(Thread)
}
