use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PagedResult<T: Debug> {
  pub items: Vec<T>,
  pub next:  Option<u64>
}

impl<T: Debug> PagedResult<T> {
  pub fn inner_into<Ty: Debug>(self) -> PagedResult<Ty>
  where
    T: Into<Ty>
  {
    PagedResult::<Ty> {
      items: self.items.into_iter().map(|i| i.into()).collect(),
      next:  self.next
    }
  }
}
