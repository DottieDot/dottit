use super::UuidString;

#[derive(Clone, Debug)]
pub struct Thread {
  pub id:    UuidString,
  pub board: String,
  pub user:  String,
  pub title: String,
  pub text:  Option<String>,
  pub media: Option<String>,
  pub score: i32
}
