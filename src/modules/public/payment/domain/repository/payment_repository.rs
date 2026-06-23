use async_trait::async_trait;

use crate::{modules::public::payment::domain::entity::service_payment::ServicePayment, shared::infra::error::AppError};

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn create_service_payment(&self, service_payment: ServicePayment) -> Result<(), AppError>;
    async fn find_by_service_information_id(&self, service_information_id: String, kind: String) -> Result<Option<ServicePayment>, AppError>;
    async fn find_by_payment_link(&self, payment_link: String) -> Result<Option<ServicePayment>, AppError>;
    async fn update_payment(&self, payment_service_scheduled: ServicePayment) -> Result<(), AppError>;
}