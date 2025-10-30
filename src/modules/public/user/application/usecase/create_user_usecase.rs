use std::{sync::Arc};

use reqwest::StatusCode;

use crate::{modules::public::user::{UserAppState, application::{dto::create_user_dto::CreateUserDto, mapper::ApplicationMapper, service::api_cep_validator::ApiCepValidator}, domain::{repository::user_repository::UserRepository, usecase::UseCase}, infrastructure::service::api_cep_validator_impl::ApiCepValidatorImpl}, shared::infra::error::AppError};

pub struct CreateUserUseCase;

impl UseCase<CreateUserDto, Result<String, AppError>> for CreateUserUseCase {
    async fn execute(&self, input: CreateUserDto, s: Arc<UserAppState>) -> Result<String, AppError> {
        let api_cep_validator = ApiCepValidatorImpl::new();
        
        let address = api_cep_validator.validate(&input.address)
            .await
            .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, format!("Problem with address validation: {}", e.to_string())))?;
      
        let user = ApplicationMapper::to_domain_user(input);
        
        s.user_repository.create(user, address).await?;
        Ok(String::from("Created!"))
    }
}