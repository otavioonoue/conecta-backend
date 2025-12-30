use async_trait::async_trait;
use reqwest::StatusCode;

use crate::{modules::public::user::{UserAppState, application::{dto::create_user_dto::CreateUserDto, mapper::ApplicationMapper, service::api_cep_validator::ApiCepValidator, usecase::UseCase}, infrastructure::service::api_cep_validator_impl::ApiCepValidatorImpl}, shared::infra::error::AppError};
pub struct CreateUserUseCase;

#[async_trait]
impl UseCase<CreateUserDto, Result<String, AppError>> for CreateUserUseCase {
	async fn execute(&self, input: CreateUserDto, s: UserAppState) -> Result<String, AppError> {
		let api_cep_validator = ApiCepValidatorImpl::new();
		
		let address = api_cep_validator.validate(&input.address)
		    .await
		    .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, format!("Problem with address validation: {}", e.to_string())))?;
		
		let mut user = ApplicationMapper::to_domain_user(input);
		user.password = s.hash_service.hash(&user.password);
		
		s.user_repository.create(user, address).await?;
		Ok(String::from("Created!"))
	}
}