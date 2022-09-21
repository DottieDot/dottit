use super::GuidString;

#[derive(Clone, Debug)]
pub struct Thread {
  pub id:    GuidString,
  pub board: String,
  pub user:  String,
  pub title: String,
  pub text:  String,
  pub media: String,
  pub score: i32
}
