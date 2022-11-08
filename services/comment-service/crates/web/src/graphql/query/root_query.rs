use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use service::services::traits::{CommentService, GetCommentByIdError};

use super::comment_query::CommentQuery;

pub struct Query;

#[Object]
impl Query {
  async fn get_comment_by_id(
    &self,
    ctx: &Context<'_>,
    thread_id: ID
  ) -> Result<CommentQuery, GetCommentByIdError> {
    self.find_comment_by_id(ctx, thread_id).await
  }

  #[graphql(entity)]
  async fn find_comment_by_id(
    &self,
    ctx: &Context<'_>,
    #[graphql(key)] id: ID
  ) -> Result<CommentQuery, GetCommentByIdError> {
    let service = ctx.data::<Arc<dyn CommentService>>().unwrap();

    service.get_comment_by_id(&id).await.map(CommentQuery::from)
  }
}
