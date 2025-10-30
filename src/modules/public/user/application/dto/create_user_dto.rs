use serde::{Deserialize};

use crate::modules::public::user::application::dto::create_address_dto::CreateAddressDto;

#[derive(Deserialize, Clone)]
pub struct CreateUserDto {
  pub name: String,
  pub email: String,
  pub phone: String,
  pub cpf: String,
  pub password: String,
  pub address: CreateAddressDto
}