use crate::modules::public::service::{application::dto::create_service_dto::CreateServiceDto, domain::entity::service::Service};
use rust_decimal::prelude::ToPrimitive;
pub struct ApplicationMapper;

impl ApplicationMapper {
  pub fn to_domain_service(dto: CreateServiceDto) -> Service {
    let service = Service {
      id: String::from(""),
      name: dto.name,
      travel_cost: dto.travel_cost.to_i64().unwrap_or_default(),
      created_at: 0
    };
    
    return service;
  }
}