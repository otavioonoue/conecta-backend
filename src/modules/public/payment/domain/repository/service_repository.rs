use async_trait::async_trait;

use crate::{modules::public::payment::domain::entity::{service::Service, service_information::ServiceInformation}, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
    async fn find_service_information_by_id(&self, service_information_id: String) -> Result<Option<ServiceInformation>, AppError>;
    async fn update_service_information(&self, service_information: ServiceInformation) -> Result<(), AppError>;
}