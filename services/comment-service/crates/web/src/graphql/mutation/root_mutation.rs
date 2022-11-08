use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use service::{
  models::dtos::CreateCommentDto,
  services::traits::{CommentService, CreateCommentError, DeleteCommentError}
};

use crate::graphql::query::CommentQuery;

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_comment(
    &self,
    ctx: &Context<'_>,
    thread_id: ID,
    user: String,
    text: String
  ) -> Result<CommentQuery, CreateCommentError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service
      .create_comment(CreateCommentDto {
        thread_id: thread_id.into(),
        user,
        text
      })
      .await
      .map(CommentQuery::from)
  }

  pub async fn delete_comment(
    &self,
    ctx: &Context<'_>,
    comment_id: ID
  ) -> Result<bool, DeleteCommentError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service.delete_comment(&comment_id).await.map(|_| true)
  }
}
