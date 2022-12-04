use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct AlreadyLoggedIn {
  message: String
}

impl AlreadyLoggedIn {
  pub fn new() -> Self {
    Self {
      message: "you are already logged in.".into()
    }
  }
}
