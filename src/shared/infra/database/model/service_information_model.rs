use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct ServiceInformationModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub service_id: Uuid,
    pub consultant_id: Option<Uuid>,
    pub service_step_id: i16,
    pub address_id: Uuid,
    pub created_at: DateTime<Utc>
}