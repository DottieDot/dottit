use super::UserDto;

pub struct AuthenticatedUserDto {
  pub user:      UserDto,
  pub api_token: String
}
