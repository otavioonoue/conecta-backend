use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ServiceScheduleModel {
    pub id: Uuid,
    pub service_id: Uuid,
    pub user_id: Uuid,
    pub consultant_id: Uuid,
    pub service_status_id: i8,
    pub description: String,
    pub address_id: Uuid,
    pub scheduled_to: NaiveDateTime,
    pub scheduled_at: NaiveDateTime
}