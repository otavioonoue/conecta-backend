use async_trait::async_trait;

use crate::{modules::public::consultant::{ application::{dto::create_consultant_dto::CreateConsultantDto, mapper::ApplicationMapper, usecase::usecase::UseCase}, appstate::ConsultantAppState}, shared::infra::error::AppError};

pub struct CreateConsultantUseCase;

#[async_trait]
impl UseCase<CreateConsultantDto, Result<String, AppError>> for CreateConsultantUseCase {
  async fn execute(&self, input: CreateConsultantDto, s: ConsultantAppState) -> Result<String, AppError> {
    let mut consultant = ApplicationMapper::to_domain_consultant(input);
    consultant.password = s.hash_service.hash(&consultant.password);
    
    s.consultant_repository.create(consultant).await?;
    
    Ok(String::from("Created!"))
  }
}