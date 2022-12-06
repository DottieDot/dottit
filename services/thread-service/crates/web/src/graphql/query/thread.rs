use async_graphql::Object;
use chrono::{DateTime, Utc};
use thread_service_model::models::Thread as ThreadModel;
use uuid::Uuid;

use super::Board;

#[derive(Debug)]
pub struct Thread {
  thread: ThreadModel
}

#[Object]
impl Thread {
  async fn id(&self) -> Uuid {
    self.thread.id
  }

  async fn board(&self) -> Board {
    Board::new(self.thread.board_id)
  }

  async fn user(&self) -> Uuid {
    self.thread.user_id
  }

  async fn title(&self) -> &String {
    &self.thread.title
  }

  async fn text(&self) -> &String {
    &self.thread.text
  }

  async fn created_at(&self) -> DateTime<Utc> {
    self.thread.created_at
  }
}

impl From<ThreadModel> for Thread {
  fn from(thread: ThreadModel) -> Self {
    Self { thread }
  }
}
