use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct UserModel {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub cpf: String,
  pub active: bool,
  pub password: String,
  pub created_at: NaiveDateTime
}