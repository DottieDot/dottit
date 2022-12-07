#[derive(Clone)]
pub struct Pagination<T: Clone> {
  pub first: T,
  pub count: u64
}

pub struct Page<Item, Key> {
  pub items: Vec<Item>,
  pub next:  Option<Key>,
  pub total: Option<u64>
}

impl<T, Key> Page<T, Key> {
  pub fn inner_into<R>(self) -> Page<R, Key>
  where
    T: Into<R>
  {
    Page::<R, Key> {
      items: self.items.into_iter().map(|i| i.into()).collect(),
      next:  self.next,
      total: self.total
    }
  }
}
