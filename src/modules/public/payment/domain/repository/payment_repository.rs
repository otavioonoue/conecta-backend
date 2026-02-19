use async_trait::async_trait;

use crate::{modules::public::{payment::domain::entity::payment_scheduled::PaymentServiceScheduled}, shared::infra::error::AppError};

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn create_payment_service_scheduled(&self, payment_service_scheduled: PaymentServiceScheduled) -> Result<(), AppError>;
    async fn find_by_service_information_id(&self, service_information_id: String) -> Result<Option<PaymentServiceScheduled>, AppError>;
    async fn find_by_payment_link(&self, payment_link: String) -> Result<Option<PaymentServiceScheduled>, AppError>;
    async fn update_payment(&self, payment_service_scheduled: PaymentServiceScheduled ) -> Result<(), AppError>;
}