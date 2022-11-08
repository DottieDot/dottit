use async_trait::async_trait;
use model::models::{Comment, PagedResult, Pagination, UuidStr, UuidString};

use super::RepositoryResult;

#[async_trait]
pub trait CommentRepository: Send + Sync {
  async fn get_comment_by_id(&self, id: &UuidStr) -> RepositoryResult<Comment>;

  async fn get_comments_by_thread_id(
    &self,
    thread_id: &UuidStr,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Comment>>;

  async fn create_comment(
    &self,
    thread_id: UuidString,
    user: String,
    text: String
  ) -> RepositoryResult<Comment>;

  async fn delete_comment(&self, comment: &UuidStr) -> RepositoryResult<()>;
}
