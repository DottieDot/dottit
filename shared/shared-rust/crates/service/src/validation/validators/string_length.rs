use std::ops::{Bound, RangeBounds};

use crate::validation::FieldValidator;

pub trait StringLength {
  fn length_range(self, range: impl RangeBounds<usize>) -> Self;
  fn min_length(self, min: usize) -> Self;
  fn max_length(self, max: usize) -> Self;
}

impl<'a> StringLength for FieldValidator<'a, String> {
  fn length_range(mut self, range: impl RangeBounds<usize>) -> Self {
    if !range.contains(&self.value.len()) {
      let min_len: Option<usize> = match range.start_bound() {
        Bound::Included(v) => Some(*v),
        Bound::Excluded(v) => Some(v + 1),
        Bound::Unbounded => None
      };

      let max_len: Option<usize> = match range.end_bound() {
        Bound::Included(v) => Some(*v),
        Bound::Excluded(v) => Some(v - 1),
        Bound::Unbounded => None
      };

      let message = match (min_len, max_len) {
        (None, Some(max)) => format!("must have a maximum of {max} characters."),
        (Some(min), None) => format!("must have a minimum of {min} characters."),
        (Some(min), Some(max)) => {
          format!("must have a minimum of {min} and a maximum of {max} characters.")
        }
        (None, None) => format!("must be empty.")
      };

      self.errors.push(message);
    }

    self
  }

  fn min_length(self, min: usize) -> Self {
    self.length_range(min..)
  }

  fn max_length(self, max: usize) -> Self {
    self.length_range(..=max)
  }
}
