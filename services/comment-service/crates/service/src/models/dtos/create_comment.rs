use uuid::Uuid;

pub struct CreateCommentDto {
  pub thread_id: Uuid,
  pub user_id:   Uuid,
  pub text:      String
}
