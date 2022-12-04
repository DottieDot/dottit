use async_graphql::{ComplexObject, SimpleObject};
use shared_service::validation::{ValidationError as ServiceValidationError, ValidationErrorField};

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct FieldError {
  pub field:  String,
  pub errors: Vec<String>
}

#[derive(SimpleObject)]
#[graphql(shareable, complex)]
pub struct ValidationError {
  errors: Vec<FieldError>
}

#[ComplexObject]
impl ValidationError {
  pub async fn message(&self) -> &'static str {
    "One or more fields didn't pass validation."
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
