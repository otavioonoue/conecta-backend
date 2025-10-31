use rust_decimal::Decimal;
use serde::{Deserialize};
use validator::Validate;
use crate::shared::infra::validate::validators::name_validation;

#[derive(Deserialize, Clone, Validate)]
pub struct CreateServiceDto {
  #[validate(custom(function = name_validation))]
  pub name: String,
  
  pub travel_cost: Decimal,
}