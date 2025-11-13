use serde::Serialize;

#[derive(Serialize)]
pub struct Consultant {
  pub id: String,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub password: String,
  pub active: bool,
  pub created_at: i64
}