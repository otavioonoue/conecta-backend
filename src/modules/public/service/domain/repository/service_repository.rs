use async_trait::async_trait;

use crate::{modules::public::service::domain::entity::service::Service, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
  async fn create(&self, user: Service) -> Result<(), AppError>;
  async fn find_all(&self) -> Result<Vec<Service>, AppError>;
}