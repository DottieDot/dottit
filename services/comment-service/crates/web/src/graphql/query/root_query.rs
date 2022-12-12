use std::sync::Arc;

use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use comment_service_service::services::traits::{
  CommentService, GetCommentByIdError, GetCommentsByThreadIdError
};
use shared_service::model::Pagination;
use uuid::Uuid;

use super::{comment::Comment, paged::Paged, thread::Thread, User};

pub struct Query;

#[Object]
impl Query {
  async fn get_comments_by_thread(
    &self,
    ctx: &Context<'_>,
    thread_id: Uuid,
    first: DateTime<Utc>,
    count: u64
  ) -> Result<Paged<Comment, DateTime<Utc>>, GetCommentsByThreadIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    Ok(
      service
        .get_comments_by_thread_id(thread_id, Pagination { first, count })
        .await?
        .inner_into::<Comment>()
        .into()
    )
  }

  async fn get_comment(
    &self,
    ctx: &Context<'_>,
    comment_id: Uuid
  ) -> Result<Comment, GetCommentByIdError> {
    self.find_comment_by_id(ctx, comment_id).await
  }

  #[graphql(entity)]
  async fn find_comment_by_id(
    &self,
    ctx: &Context<'_>,
    id: Uuid
  ) -> Result<Comment, GetCommentByIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service.get_comment_by_id(id).await.map(Comment::from)
  }

  #[graphql(entity)]
  async fn get_thread_by_id(&self, id: Uuid) -> Thread {
    Thread { id }
  }

  #[graphql(entity)]
  async fn get_user_by_id(&self, id: Uuid) -> User {
    User { id }
  }
}
