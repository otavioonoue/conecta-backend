use async_trait::async_trait;
use axum::http::StatusCode;
use crate::{modules::public::consultant::{ConsultantAppState, application::{dto::remove_service_dto::RemoveServiceDto, usecase::UseCase}}, shared::infra::error::AppError};

pub struct RemoveServiceConsultantUseCase;

#[async_trait]
impl UseCase<(RemoveServiceDto, String), Result<(), AppError>> for RemoveServiceConsultantUseCase {
    async fn execute(&self, (service_dto, consultant_id): (RemoveServiceDto, String), s: ConsultantAppState) -> Result<(), AppError> {
        if let None = s.consultant_repository.find_by_id(consultant_id.to_string()).await? {
            return Err(AppError::new(StatusCode::NOT_FOUND, String::from("The consultant doesn't exists")))
        }
        
        if let None = s.consultant_repository.find_service_by_id(service_dto.service_id.clone()).await? {
            return Err(AppError::new(StatusCode::NOT_FOUND, String::from("The service doesn't exists")))
        }
        
        let consultant_has_service = s.consultant_repository.consultant_has_service(consultant_id.to_string(), service_dto.service_id.clone()).await?;
        
        if !consultant_has_service {
            return Err(AppError::new(StatusCode::NOT_FOUND, String::from("The consultant doesn't has the service")))
        }
        
        let resp = s.consultant_repository.remove_service(consultant_id, service_dto.service_id).await?;
        
        Ok(resp)
    }
}