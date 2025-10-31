use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct ConsultantModel {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub password: String,
  pub active: bool,
  pub created_at: NaiveDateTime
}