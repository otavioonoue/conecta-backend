use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ServiceBudgetModel {
    pub id: Uuid,
    pub service_information_id: Uuid,
    pub service_cost: Decimal,
    pub travel_cost: Decimal,
    pub description: String,
    pub service_budget_status_id: i16,
    pub created_at: DateTime<Utc>
}