use async_graphql::{Object, ID};
use model::models::Thread;

pub struct ThreadQuery {
  thread: Thread
}

#[Object]
impl ThreadQuery {
  async fn thread_id(&self) -> ID {
    self.thread.id.clone().into()
  }

  async fn board(&self) -> String {
    self.thread.board.clone()
  }

  async fn user(&self) -> String {
    self.thread.user.clone()
  }

  async fn title(&self) -> String {
    self.thread.title.clone()
  }

  async fn text(&self) -> Option<String> {
    self.thread.text.clone()
  }

  async fn media(&self) -> Option<String> {
    self.thread.media.clone()
  }

  async fn score(&self) -> i32 {
    self.thread.score
  }

  async fn hello(&self) -> String {
    "Hello World!".to_owned()
  }
}

impl From<Thread> for ThreadQuery {
  fn from(thread: Thread) -> Self {
    Self { thread }
  }
}
