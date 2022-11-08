use async_graphql::{Object, ID};
use model::models::Thread as ThreadModel;

pub struct Thread {
  thread: ThreadModel
}

#[Object]
impl Thread {
  async fn id(&self) -> ID {
    self.thread.id.clone().into()
  }

  async fn board(&self) -> &String {
    &self.thread.board
  }

  async fn user(&self) -> &String {
    &self.thread.user
  }

  async fn title(&self) -> &String {
    &self.thread.title
  }

  async fn text(&self) -> &Option<String> {
    &self.thread.text
  }

  async fn media(&self) -> &Option<String> {
    &self.thread.media
  }

  async fn score(&self) -> i32 {
    self.thread.score
  }
}

impl From<ThreadModel> for Thread {
  fn from(thread: ThreadModel) -> Self {
    Self { thread }
  }
}
