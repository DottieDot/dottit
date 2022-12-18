use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct AlreadyLoggedIn {
  message: String
}

impl Default for AlreadyLoggedIn {
  fn default() -> Self {
    Self {
      message: "you are already logged in.".into()
    }
  }
}
