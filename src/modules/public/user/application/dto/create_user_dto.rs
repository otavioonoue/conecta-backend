use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct CreateUserDto {
  pub name: String,
  pub email: String,
  pub phone: String,
  pub cpf: String,
  pub password: String
}