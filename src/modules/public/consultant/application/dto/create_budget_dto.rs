use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBudgetDto {
    pub service_cost: Decimal,
    pub description: String,
}