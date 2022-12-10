use chrono::{DateTime, Utc};
use comment_service_model::models::Comment;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommentDto {
  pub id:         Uuid,
  pub thread_id:  Uuid,
  pub user_id:    Uuid,
  pub text:       String,
  pub created_at: DateTime<Utc>
}

impl From<Comment> for CommentDto {
  fn from(comment: Comment) -> Self {
    Self {
      id:         comment.id,
      thread_id:  comment.thread_id,
      user_id:    comment.user_id,
      text:       comment.text,
      created_at: comment.created_at
    }
  }
}
