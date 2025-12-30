use async_trait::async_trait;
use axum::http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{UserAppState, application::{dto::create_address_dto::CreateAddressDto, service::api_cep_validator::ApiCepValidator, usecase::UseCase}, infrastructure::service::api_cep_validator_impl::ApiCepValidatorImpl}}, shared::infra::error::AppError};

pub struct CreateAddressUseCase;

#[async_trait]
impl UseCase<(Claims, CreateAddressDto), Result<(), AppError>> for CreateAddressUseCase {
    async fn execute(&self, (user, dto): (Claims, CreateAddressDto), s: UserAppState) -> Result<(), AppError> {
        let api_cep_validator = ApiCepValidatorImpl::new();
        
        let address = api_cep_validator.validate(&dto)
            .await
            .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, format!("Problem with address validation: {}", e.to_string())))?;
        
        s.address_repository.create(user.sub, address).await?;
        Ok(())
    }
}