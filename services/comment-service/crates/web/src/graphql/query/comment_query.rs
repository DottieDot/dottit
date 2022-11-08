use async_graphql::{Object, ID};
use service::models::dtos::CommentDto;

pub struct CommentQuery {
  comment: CommentDto
}

#[Object]
impl CommentQuery {
  async fn id(&self) -> ID {
    self.comment.id.clone().into()
  }

  async fn thread_id(&self) -> ID {
    ID::from(&self.comment.thread_id)
  }

  async fn user(&self) -> &String {
    &self.comment.user
  }

  async fn text(&self) -> &String {
    &self.comment.text
  }

  async fn score(&self) -> i32 {
    self.comment.score
  }
}

impl From<CommentDto> for CommentQuery {
  fn from(comment: CommentDto) -> Self {
    Self { comment }
  }
}
