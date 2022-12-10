use async_trait::async_trait;
use chrono::{DateTime, Utc};
use comment_service_model::models::Comment;
use shared_service::model::{Page, Pagination};
use uuid::Uuid;

use super::RepositoryResult;

#[async_trait]
pub trait CommentRepository: Send + Sync {
  async fn get_comment_by_id(&self, id: Uuid) -> RepositoryResult<Comment>;

  async fn get_comments_by_thread_id(
    &self,
    thread_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> RepositoryResult<Page<Comment, DateTime<Utc>>>;

  async fn create_comment(
    &self,
    thread_id: Uuid,
    user_id: Uuid,
    text: String
  ) -> RepositoryResult<Comment>;

  async fn delete_comment(&self, comment: Uuid) -> RepositoryResult<()>;
}
