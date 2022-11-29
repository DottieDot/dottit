use async_graphql::{Object, SimpleObject};
use shared_service::validation::{ValidationError as ServiceValidationError, ValidationErrorField};

#[derive(SimpleObject)]
pub struct FieldError {
  pub field:  String,
  pub errors: Vec<String>
}

pub struct ValidationError {
  errors: Vec<FieldError>
}

#[Object]
impl ValidationError {
  pub async fn message(&self) -> &'static str {
    "One or more fields didn't pass validation."
  }

  pub async fn errors(&self) -> &Vec<FieldError> {
    &self.errors
  }
}

impl From<ValidationErrorField> for FieldError {
  fn from(error_field: ValidationErrorField) -> Self {
    Self {
      field:  error_field.field,
      errors: error_field.errors
    }
  }
}

impl From<ServiceValidationError> for ValidationError {
  fn from(error: ServiceValidationError) -> Self {
    Self {
      errors: error.errors.into_iter().map(|field| field.into()).collect()
    }
  }
}
