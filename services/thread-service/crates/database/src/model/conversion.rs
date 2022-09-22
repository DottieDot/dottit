use model::models::Thread;

use super::thread;

impl From<thread::Model> for Thread {
  fn from(thread: thread::Model) -> Self {
    Self {
      id:    thread.id.to_string(),
      board: thread.board,
      user:  thread.user,
      title: thread.title,
      text:  thread.text,
      media: thread.media,
      score: thread.score
    }
  }
}
