use serde::Serialize;

#[derive(Serialize)]
pub struct Address {
  pub id: String,
  pub cep: String,
  pub number: String,
  pub street: String,
  pub neighborhood: String,
  pub city: String,
  pub state: String,
  pub user_id: String
}