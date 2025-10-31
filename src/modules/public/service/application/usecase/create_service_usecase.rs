use async_trait::async_trait;

use crate::{modules::public::service::{ServiceAppState, application::{dto::create_service_dto::CreateServiceDto, mapper::ApplicationMapper, usecase::UseCase}}, shared::infra::error::AppError};

pub struct CreateServiceUseCase;

#[async_trait]
impl UseCase<CreateServiceDto, Result<String, AppError>> for CreateServiceUseCase {
    async fn execute(&self, input: CreateServiceDto, s: ServiceAppState) -> Result<String, AppError> {
        let service = ApplicationMapper::to_domain_service(input);
        s.service_repository.create(service).await?;
        
        Ok(String::from("Created!"))
    }
}