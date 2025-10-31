use async_trait::async_trait;

use crate::{modules::public::consultant::domain::entity::consultant::Consultant, shared::infra::error::AppError};

#[async_trait]
pub trait ConsultantRepository: Send + Sync {
  async fn create(&self, user: Consultant) -> Result<(), AppError>;
  async fn find_all(&self) -> Result<Vec<Consultant>, AppError>;
}