use serde::Serialize;

#[derive(Serialize)]
pub struct Service {
  pub id: String,
  pub name: String,
  pub travel_cost: i64,
  pub created_at: i64
}