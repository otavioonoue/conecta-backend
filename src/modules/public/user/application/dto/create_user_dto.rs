use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::public::user::application::dto::create_address_dto::CreateAddressDto;
use crate::shared::infra::validate::validators::{name_validation, email_validation, phone_validation, cpf_validation, password_validation};

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct CreateUserDto {
  #[validate(custom(function = name_validation))]
  pub name: String,
  
  #[validate(custom(function = email_validation))]
  pub email: String,
  
  #[validate(custom(function = phone_validation))]
  pub phone: String,
  
  #[validate(custom(function = cpf_validation))]
  pub cpf: String,
  
  #[validate(custom(function = password_validation))]
  pub password: String,

  #[validate(nested)]
  pub address: CreateAddressDto
}