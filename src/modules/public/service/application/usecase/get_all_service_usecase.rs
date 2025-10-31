use async_trait::async_trait;

use crate::{modules::public::service::{ ServiceAppState, application::usecase::UseCase, domain::entity::service::Service}, shared::infra::error::AppError};

pub struct GetAllServicesUseCase;

#[async_trait]
impl UseCase<(), Result<Vec<Service>, AppError>> for GetAllServicesUseCase {
  async fn execute(&self, _: (), s: ServiceAppState) -> Result<Vec<Service>, AppError> {
    let services = s.service_repository.find_all().await?;
    
    Ok(services)
  }
}