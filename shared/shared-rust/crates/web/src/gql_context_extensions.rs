use async_graphql::Context;

use crate::AuthenticatedUser;

pub trait GqlContextExtensions {
  fn authenticated_user(&self) -> Option<&AuthenticatedUser>;
}

impl<'a> GqlContextExtensions for Context<'a> {
  fn authenticated_user(&self) -> Option<&AuthenticatedUser> {
    self.data_opt::<AuthenticatedUser>()
  }
}
