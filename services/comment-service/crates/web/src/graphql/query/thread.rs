use std::sync::Arc;

use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use comment_service_service::services::traits::{CommentService, GetCommentsByThreadIdError};
use shared_service::model::Pagination;
use uuid::Uuid;

use super::{paged::Paged, Comment};

pub struct Thread {
  pub id: Uuid
}

#[Object(extends)]
impl Thread {
  #[graphql(external)]
  async fn id(&self) -> Uuid {
    self.id
  }

  async fn comments(
    &self,
    ctx: &Context<'_>,
    first: DateTime<Utc>,
    count: u64
  ) -> Result<Paged<Comment, DateTime<Utc>>, GetCommentsByThreadIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    let comments = service
      .get_comments_by_thread_id(self.id, Pagination { first, count })
      .await?
      .inner_into::<Comment>();

    Ok(comments.into())
  }
}
