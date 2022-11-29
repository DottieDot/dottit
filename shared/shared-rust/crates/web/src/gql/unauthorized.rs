use async_graphql::Object;

pub struct Unauthorized;

#[Object]
impl Unauthorized {
  pub async fn message(&self) -> &'static str {
    "You do not have access this resource."
  }
}
