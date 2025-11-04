use async_trait::async_trait;

use crate::{modules::public::consultant::domain::entity::{consultant::Consultant, service::Service}, shared::infra::error::AppError};

#[async_trait]
pub trait ConsultantRepository: Send + Sync {
  async fn create(&self, user: Consultant) -> Result<(), AppError>;
  async fn find_all(&self) -> Result<Vec<Consultant>, AppError>;
  async fn find_by_id(&self, consultant_id: String) -> Result<Option<Consultant>, AppError>;
  async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
  async fn consultant_has_service(&self, consultant_id: String, service_id: String) -> Result<bool, AppError>;
  async fn add_service(&self, consultant_id: String, service_id: String) -> Result<(), AppError>;
  async fn remove_service(&self, consultant_id: String, service_id: String) -> Result<(), AppError>;
}