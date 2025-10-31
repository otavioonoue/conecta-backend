use serde::{Deserialize};
use validator::Validate;
use crate::shared::infra::validate::validators::{email_validation, name_validation, password_validation, phone_validation};

#[derive(Deserialize, Clone, Validate)]
pub struct CreateConsultantDto {
  #[validate(custom(function = name_validation))]
  pub name: String,
  
  #[validate(custom(function = email_validation))]
  pub email: String,
  
  #[validate(custom(function = phone_validation))]
  pub phone: String,
  
  #[validate(custom(function = password_validation))]
  pub password: String,
}