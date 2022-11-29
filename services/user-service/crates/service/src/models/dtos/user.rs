use user_service_model::models::User;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserDto {
  pub id:       Uuid,
  pub username: String
}

impl From<User> for UserDto {
  fn from(user: User) -> Self {
    Self {
      id:       user.id,
      username: user.username
    }
  }
}
