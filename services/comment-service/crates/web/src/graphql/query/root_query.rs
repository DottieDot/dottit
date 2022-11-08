use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use model::models::Pagination;
use service::services::traits::{CommentService, GetCommentByIdError, GetCommentsByThreadIdError};

use super::{comment::Comment, paged::Paged, thread::Thread};

pub struct Query;

#[Object]
impl Query {
  async fn get_comments_by_thread(
    &self,
    ctx: &Context<'_>,
    thread_id: ID,
    first: u64,
    count: u64
  ) -> Result<Paged<Comment>, GetCommentsByThreadIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    Ok(
      service
        .get_comments_by_thread_id(&thread_id, Pagination { first, count })
        .await?
        .inner_into::<Comment>()
        .into()
    )
  }

  async fn get_comment(
    &self,
    ctx: &Context<'_>,
    comment_id: ID
  ) -> Result<Comment, GetCommentByIdError> {
    self.find_comment_by_id(ctx, comment_id).await
  }

  #[graphql(entity)]
  async fn find_comment_by_id(
    &self,
    ctx: &Context<'_>,
    id: ID
  ) -> Result<Comment, GetCommentByIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service.get_comment_by_id(&id).await.map(Comment::from)
  }

  #[graphql(entity)]
  async fn get_thread_by_id(&self, id: ID) -> Thread {
    Thread { id }
  }
}
