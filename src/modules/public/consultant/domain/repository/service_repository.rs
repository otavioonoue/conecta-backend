use async_trait::async_trait;

use crate::{modules::public::consultant::domain::entity::{service::Service, service_budget::ServiceBudget, service_information::ServiceInformation, service_order::ServiceOrder}, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
    async fn find_service_information_by_id(&self, service_information_id: String) -> Result<Option<ServiceInformation>, AppError>;
    async fn update_service_information(&self, service_information: ServiceInformation) -> Result<(), AppError>;
    async fn create_service_budget(&self, service_budget: ServiceBudget) -> Result<(), AppError>;
    async fn create_service_order(&self, service_order: ServiceOrder) -> Result<(), AppError>;
    async fn find_budgets_approved_by_service_information_id(&self, service_information_id: String) -> Result<Vec<ServiceBudget>, AppError>;
    async fn find_service_order_by_service_information_id(&self, service_information_id: String) -> Result<Option<ServiceOrder>, AppError>;
    async fn update_service_order_status(&self, service_order: ServiceOrder) -> Result<(), AppError>;
}