use async_trait::async_trait;

use crate::{modules::public::service::domain::entity::{service::Service, service_schedule::ServiceSchedule}, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn create(&self, user: Service) -> Result<(), AppError>;
    async fn find_all(&self) -> Result<Vec<Service>, AppError>;
    async fn find_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
    
    async fn schedule(&self, service_schedule: ServiceSchedule) -> Result<(), AppError>;
    async fn find_by_user(&self, user_id: String) -> Result<(), AppError>;
}