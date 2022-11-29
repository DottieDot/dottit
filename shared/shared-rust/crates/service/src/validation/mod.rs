pub mod validators;

use std::marker::PhantomData;

use async_trait::async_trait;

#[async_trait]
pub trait Validator<T> {
  async fn validate(&self, object: &T) -> Result<(), ValidationError>;
}

pub struct ValidationError {
  pub errors: Vec<ValidationErrorField>
}

#[derive(Default)]
pub struct ValidationResultBuilder {
  errors: Vec<ValidationErrorField>
}

impl ValidationResultBuilder {
  pub fn field<T>(mut self, validator: FieldValidator<'_, T>) -> Self {
    if let Some(error) = validator.error_field() {
      self.errors.push(error);
    }
    self
  }

  pub fn finish(self) -> Result<(), ValidationError> {
    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(ValidationError {
        errors: self.errors
      })
    }
  }
}

pub struct ValidationErrorField {
  pub field:  String,
  pub errors: Vec<String>
}

pub struct FieldValidator<'a, T> {
  field:  String,
  value:  &'a T,
  errors: Vec<String>,
  pd:     PhantomData<T>
}

impl<'a, T> FieldValidator<'a, T> {
  pub fn new(field: impl Into<String>, value: &'a T) -> Self {
    Self {
      field: field.into(),
      value,
      errors: Vec::new(),
      pd: Default::default()
    }
  }

  pub fn add_error(&mut self, error: String) {
    self.errors.push(error)
  }

  pub fn error_field(self) -> Option<ValidationErrorField> {
    (!self.errors.is_empty()).then_some(ValidationErrorField {
      field:  self.field,
      errors: self.errors
    })
  }
}
