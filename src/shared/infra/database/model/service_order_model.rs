use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ServiceOrderModel {
    pub id: Uuid,
    pub service_information_id: Uuid,
    pub final_cost: Decimal,
    pub description: String,
    pub service_order_status_id: i16,
    pub scheduled_to: DateTime<Utc>,
    pub scheduled_at: DateTime<Utc>
}