use async_trait::async_trait;

use crate::{modules::public::consultant::{ConsultantAppState, application::{dto::create_consultant_dto::CreateConsultantDto, mapper::ApplicationMapper, usecase::UseCase}}, shared::infra::error::AppError};

pub struct CreateConsultantUseCase;

#[async_trait]
impl UseCase<CreateConsultantDto, Result<String, AppError>> for CreateConsultantUseCase {
  async fn execute(&self, input: CreateConsultantDto, s: ConsultantAppState) -> Result<String, AppError> {
    let consultant = ApplicationMapper::to_domain_consultant(input);
    s.consultant_repository.create(consultant).await?;
    
    Ok(String::from("Created!"))
  }
}