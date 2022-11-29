use std::str::FromStr;

use async_graphql::{Object, ID};
use comment_service_service::models::dtos::CommentDto;
use uuid::Uuid;

use super::thread::Thread;

#[derive(Debug)]
pub struct Comment {
  comment: CommentDto
}

#[Object]
impl Comment {
  async fn id(&self) -> ID {
    self.comment.id.clone().into()
  }

  async fn thread_id(&self) -> ID {
    ID::from(&self.comment.thread_id)
  }

  async fn thread(&self) -> Thread {
    Thread {
      id: Uuid::from_str(&self.comment.thread_id).unwrap()
    }
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

impl From<CommentDto> for Comment {
  fn from(comment: CommentDto) -> Self {
    Self { comment }
  }
}
