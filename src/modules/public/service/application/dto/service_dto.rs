use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceDto {
	pub id: String,
	pub name: String,
	pub travel_cost: Decimal,
	pub created_at: i64
}