use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PaymentServiceScheduled {
    pub id: String,
    pub schedule_service_information_id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_payment_id: String,
    pub status: String,
    pub cost: i64,
    pub created_at: i64
}