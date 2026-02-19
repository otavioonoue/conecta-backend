use async_trait::async_trait;

use crate::{modules::public::user::domain::entity::{service::Service, service_budget::ServiceBudget, service_information::ServiceInformation}, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
    async fn find_service_information_by_service_budget_id(&self, service_budget_id: String) -> Result<Option<ServiceInformation>, AppError>;
    async fn update_service_information(&self, service_information: ServiceInformation) -> Result<(), AppError>;
    async fn create_service_budget(&self, service_budget: ServiceBudget) -> Result<(), AppError>;
    async fn find_service_budget_by_id(&self, service_budget_id: String) -> Result<Option<ServiceBudget>, AppError>;
    async fn update_service_budget_status(&self, service_budget: ServiceBudget) -> Result<(), AppError>;
}