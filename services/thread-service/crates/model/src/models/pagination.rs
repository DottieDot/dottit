#[derive(Clone, Copy, Debug)]
pub struct Pagination {
  pub first: u64,
  pub count: u64
}

impl Pagination {
  pub fn next(&self, received_len: usize) -> Option<u64> {
    if received_len == (self.count + 1).try_into().unwrap() {
      Some(self.first + self.count)
    } else {
      None
    }
  }
}
