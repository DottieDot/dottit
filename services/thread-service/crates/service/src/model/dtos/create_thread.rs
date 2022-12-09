use uuid::Uuid;

pub struct CreateThreadDto {
  pub board_id: Uuid,
  pub user_id:  Uuid,
  pub title:    String,
  pub text:     String
}
