use async_graphql::Object;
use chrono::{DateTime, Utc};
use comment_service_service::models::dtos::CommentDto;
use uuid::Uuid;

use super::{thread::Thread, User};

#[derive(Debug)]
pub struct Comment {
  comment: CommentDto
}

#[Object]
impl Comment {
  async fn id(&self) -> Uuid {
    self.comment.id
  }

  async fn thread(&self) -> Thread {
    Thread {
      id: self.comment.thread_id
    }
  }

  async fn user(&self) -> User {
    User {
      id: self.comment.user_id
    }
  }

  async fn text(&self) -> &String {
    &self.comment.text
  }

  async fn created_at(&self) -> DateTime<Utc> {
    self.comment.created_at
  }
}

impl From<CommentDto> for Comment {
  fn from(comment: CommentDto) -> Self {
    Self { comment }
  }
}
