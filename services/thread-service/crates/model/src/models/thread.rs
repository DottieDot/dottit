use super::GuidString;

#[derive(Clone, Debug)]
pub struct Thread {
  pub id:    GuidString,
  pub board: String,
  pub user:  String,
  pub title: String,
  pub text:  Option<String>,
  pub media: Option<String>,
  pub score: i32
}
