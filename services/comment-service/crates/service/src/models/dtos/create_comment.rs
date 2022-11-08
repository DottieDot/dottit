use model::models::UuidString;

pub struct CreateCommentDto {
  pub thread_id: UuidString,
  pub user:      String,
  pub text:      String
}
