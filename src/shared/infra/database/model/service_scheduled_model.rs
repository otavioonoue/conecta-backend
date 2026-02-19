use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ServiceScheduleModel {
    pub id: Uuid,
    pub service_information_id: Uuid,
    pub service_status_id: i8,
    pub description: String,
    pub scheduled_to: DateTime<Utc>,
    pub scheduled_at: DateTime<Utc>
}