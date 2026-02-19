use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PaymentServiceScheduledModel {
    pub id: Uuid,
    pub schedule_service_information_id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_payment_id: String,
    pub status: String,
    pub cost: Decimal,
    pub created_at: DateTime<Utc>
}