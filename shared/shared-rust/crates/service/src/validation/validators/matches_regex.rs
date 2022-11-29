use crate::validation::FieldValidator;

pub trait MatchesRegex {
  fn matches(self, regex: &regex::Regex, message: impl Into<String>) -> Self;
}

impl<'a> MatchesRegex for FieldValidator<'a, String> {
  fn matches(mut self, regex: &regex::Regex, message: impl Into<String>) -> Self {
    if !regex.is_match(&self.value) {
      self.errors.push(message.into());
    }

    self
  }
}
