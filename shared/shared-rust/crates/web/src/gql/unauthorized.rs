use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Unauthorized {
  message: String
}

impl Unauthorized {
  pub fn new() -> Self {
    Self {
      message: "you do not have access this resource.".into()
    }
  }
}
