use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct CreateConsultantDto {
  pub name: String,
  pub email: String,
  pub phone: String,
  pub password: String,
}