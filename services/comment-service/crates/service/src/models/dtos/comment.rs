use model::models::{Comment, UuidString};

#[derive(Debug)]
pub struct CommentDto {
  pub id:        UuidString,
  pub thread_id: UuidString,
  pub user:      String,
  pub text:      String,
  pub score:     i32
}

impl From<Comment> for CommentDto {
  fn from(comment: Comment) -> Self {
    Self {
      id:        comment.id,
      thread_id: comment.thread_id,
      user:      comment.user,
      text:      comment.text,
      score:     comment.score
    }
  }
}
