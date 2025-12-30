use crate::{modules::public::service::{application::dto::create_service_dto::CreateServiceDto, domain::entity::service::Service}, shared::infra::helpers::currency::CurrencyHelper};
pub struct ApplicationMapper;

impl ApplicationMapper {
	pub fn to_domain_service(dto: CreateServiceDto) -> Service {
		let travel_cost_cents = CurrencyHelper::to_cents(dto.travel_cost);
		
		let service = Service {
			id: String::from(""),
			name: dto.name,
			travel_cost: travel_cost_cents,
			created_at: 0
		};
	
		return service;
	}
}