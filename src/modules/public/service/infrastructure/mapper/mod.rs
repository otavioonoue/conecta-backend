use std::str::FromStr;

use chrono::DateTime;
use rust_decimal::Decimal;
use sqlx::types::Uuid;

use crate::{
    modules::public::service::domain::entity::{address::Address, consultant::Consultant, service::Service},
    shared::infra::{database::model::{address_model::AddressModel, consultant_model::ConsultantModel, service_model::ServiceModel}, helpers::currency::CurrencyHelper},
};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
	pub fn to_data_service(service: Service) -> ServiceModel {
        ServiceModel {
            id: Uuid::from_str(&service.id).unwrap_or_default(),
            name: service.name,
            travel_cost: Decimal::new(service.travel_cost, 2),
            created_at: DateTime::from_timestamp(service.created_at, 0)
                .unwrap()
                .naive_utc(),
        }
    }
    
    pub fn to_domain_service(service_data: ServiceModel) -> Service {
		let travel_cost_cents = CurrencyHelper::to_cents(service_data.travel_cost);
		Service {
			id: service_data.id.to_string(),
			name: service_data.name,
			travel_cost: travel_cost_cents,
			created_at: service_data.created_at.and_utc().timestamp(),
		}
    }
    
    pub fn to_data_consultant(consultant: Consultant) -> ConsultantModel {
        ConsultantModel {
            id: Uuid::from_str(&consultant.id).unwrap_or_default(),
            name: consultant.name,
            email: consultant.email,
            phone: consultant.phone,
            password: consultant.password,
            active: consultant.active,
            created_at: DateTime::from_timestamp(consultant.created_at, 0)
                .unwrap()
                .naive_utc(),
        }
    }
    
    pub fn to_domain_consultant(consultant_data: ConsultantModel) -> Consultant {
        Consultant {
            id: consultant_data.id.to_string(),
            name: consultant_data.name,
            email: consultant_data.email,
            phone: consultant_data.phone,
            password: consultant_data.password,
            active: consultant_data.active,
            created_at: consultant_data.created_at.and_utc().timestamp(),
        }
    }
    
    pub fn to_domain_address(address_data: AddressModel) -> Address {
        Address {
            id: address_data.id.to_string(), 
            cep: address_data.cep, 
            number: address_data.number, 
            street: address_data.street, 
            neighborhood: address_data.neighborhood, 
            city: address_data.city, 
            state: address_data.state,
            user_id: address_data.user_id.to_string()
        }
    }
}
