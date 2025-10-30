use serde::Serialize;

#[derive(Serialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub cpf: String,
  pub active: bool,
  pub password: String,
  pub created_at: i64
}