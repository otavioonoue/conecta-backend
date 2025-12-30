use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::{modules::public::service::{application::{dto::service_dto::ServiceDto, usecase::UseCase}, appstate::ServiceAppState}, shared::infra::error::AppError};

pub struct GetAllServicesUseCase;

#[async_trait]
impl UseCase<(), Result<Vec<ServiceDto>, AppError>> for GetAllServicesUseCase {
	async fn execute(&self, _: (), s: ServiceAppState) -> Result<Vec<ServiceDto>, AppError> {
		let services = s.service_repository.find_all().await?;
		
		let services_dtos: Vec<ServiceDto> = services.iter().map(|s| ServiceDto {
			id: s.id.clone(),
			name: s.name.clone(),
			travel_cost: Decimal::new(s.travel_cost, 2),
			created_at: s.created_at
		})
		.collect();
		
		Ok(services_dtos)
	}
}