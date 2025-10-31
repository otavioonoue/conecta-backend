use async_trait::async_trait;

use crate::{modules::public::consultant::{ConsultantAppState, application::usecase::UseCase, domain::{entity::consultant::Consultant}}, shared::infra::error::AppError};

pub struct GetAllConsultantsUseCase;

#[async_trait]
impl UseCase<(), Result<Vec<Consultant>, AppError>> for GetAllConsultantsUseCase {
  async fn execute(&self, _: (), s: ConsultantAppState) -> Result<Vec<Consultant>, AppError> {
    let consultants = s.consultant_repository.find_all().await?;
    
    Ok(consultants)
  }
}