use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct AddressModel {
  pub id: Uuid,
  pub cep: String,
  pub number: String,
  pub street: String,
  pub neighborhood: String,
  pub city: String,
  pub state: String,
  pub user_id: Uuid
}