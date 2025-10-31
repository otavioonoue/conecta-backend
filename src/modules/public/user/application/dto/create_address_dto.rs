use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::infra::validate::validators::{cep_validation, number_validation};

#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct CreateAddressDto {
  #[validate(custom(function = cep_validation))]
  pub cep: String,
  
  #[validate(custom(function = number_validation))]
  pub number: String
}