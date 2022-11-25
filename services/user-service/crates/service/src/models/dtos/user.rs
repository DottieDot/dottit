use model::models::{User, UuidString};

#[derive(Debug, Clone)]
pub struct UserDto {
  pub id:       UuidString,
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
