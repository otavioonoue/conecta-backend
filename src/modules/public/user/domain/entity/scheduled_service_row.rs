use serde::Serialize;

#[derive(Serialize)]
pub struct ScheduledServiceRow {
    pub id: String,
    pub service_information_id: String,
    pub service_status_id: i32,
    pub description: String,
    pub scheduled_at: i64,
    pub scheduled_to: i64,
    pub service_name: String,
    pub travel_cost: i64,
    pub service_step_id: i16,
    pub street: String,
    pub number: String,
    pub neighborhood: String,
    pub city: String,
    pub cep: String
}