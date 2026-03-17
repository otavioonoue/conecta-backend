use serde::Serialize;

use crate::shared::infra::database::model::service_payment_model::PaymentKindModel;

#[derive(Serialize, Clone)]
pub struct ServicePayment {
    pub id: String,
    pub schedule_service_information_id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_payment_id: String,
    pub kind: PaymentKind,
    pub status: String,
    pub cost: i64,
    pub created_at: i64
}

#[derive(Serialize, Clone)]
pub enum PaymentKind {
    Budget,
    Scheduled
}

impl From<PaymentKind> for PaymentKindModel {
    fn from(value: PaymentKind) -> Self {
        match value {
            PaymentKind::Budget => Self::Budget,
            PaymentKind::Scheduled => Self::Scheduled,
        }
    }
}