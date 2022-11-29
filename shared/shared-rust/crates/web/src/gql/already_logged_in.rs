use async_graphql::Object;

pub struct AlreadyLoggedIn;

#[Object]
impl AlreadyLoggedIn {
  pub async fn message(&self) -> &'static str {
    "you are already logged in."
  }
}
