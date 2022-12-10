use std::sync::Arc;

use async_graphql::{Context, Object, Union};
use comment_service_service::{
  models::dtos::CreateCommentDto,
  services::traits::{CommentService, CreateCommentError, DeleteCommentError}
};
use shared_web::{
  gql::{Unauthenticated, ValidationError},
  GqlContextExtensions
};
use uuid::Uuid;

use crate::graphql::query::Comment;

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_comment(
    &self,
    ctx: &Context<'_>,
    thread_id: Uuid,
    text: String
  ) -> Result<CreateCommentResult, CreateCommentError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();
    let Some(user) = ctx.authenticated_user() else {
      return Ok(CreateCommentResult::Unauthenticated(Unauthenticated::new()));
    };

    let result = service
      .create_comment(CreateCommentDto {
        thread_id: thread_id.into(),
        user_id: user.user_id,
        text
      })
      .await?;

    match result {
      Ok(comment) => Ok(CreateCommentResult::Created(comment.into())),
      Err(e) => Ok(CreateCommentResult::ValidationError(e.into()))
    }
  }

  pub async fn delete_comment(
    &self,
    ctx: &Context<'_>,
    comment_id: Uuid
  ) -> Result<bool, DeleteCommentError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service.delete_comment(comment_id).await.map(|_| true)
  }
}

#[derive(Union)]
pub enum CreateCommentResult {
  Unauthenticated(Unauthenticated),
  ValidationError(ValidationError),
  Created(Comment)
}
