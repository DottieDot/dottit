use super::UuidString;

#[derive(Clone, Debug)]
pub struct Comment {
  pub id:        UuidString,
  pub thread_id: UuidString,
  pub user:      String,
  pub text:      String,
  pub score:     i32
}
