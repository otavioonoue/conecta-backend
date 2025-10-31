use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct ServiceModel {
  pub id: Uuid,
  pub name: String,
  pub travel_cost: Decimal,
  pub created_at: NaiveDateTime
}