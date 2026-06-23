use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ScheduledServiceRowModel {
    pub id: Uuid,
    pub service_information_id: Uuid,
    pub service_status_id: i32,
    pub description: String,
    pub scheduled_at: DateTime<Utc>,
    pub scheduled_to: DateTime<Utc>,
    pub service_name: String,
    pub travel_cost: Decimal,
    pub service_step_id: i16,
    pub street: String,
    pub number: String,
    pub neighborhood: String,
    pub city: String,
    pub cep: String
}