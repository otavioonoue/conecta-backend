use std::str::FromStr;

use chrono::DateTime;
use rust_decimal::Decimal;
use sqlx::types::Uuid;

use crate::{
    modules::public::user::domain::entity::{address::Address, service::Service, service_budget::ServiceBudget, service_information::ServiceInformation, user::User},
    shared::infra::{database::model::{address_model::AddressModel, service_budget_model::ServiceBudgetModel, service_information_model::ServiceInformationModel, service_model::ServiceModel, user_model::UserModel}, helpers::currency::CurrencyHelper},
};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
    pub fn to_data_user(user: User) -> UserModel {
        UserModel {
            id: Uuid::from_str(&user.id).unwrap_or_default(),
            name: user.name,
            email: user.email,
            phone: user.phone,
            cpf: user.cpf,
            active: user.active,
            password: user.password,
            created_at: DateTime::from_timestamp(user.created_at, 0)
                .unwrap(),
        }
    }

    pub fn to_domain_user(user_data: UserModel) -> User {
        User {
            id: user_data.id.to_string(),
            name: user_data.name,
            email: user_data.email,
            phone: user_data.phone,
            cpf: user_data.cpf,
            active: user_data.active,
            password: user_data.password,
            created_at: user_data.created_at.timestamp(),
        }
    }

    pub fn to_data_address(address: Address, user_id: String) -> AddressModel {
        AddressModel {
            id: Uuid::from_str(&address.id).unwrap_or_default(),
            cep: address.cep,
            number: address.number,
            street: address.street,
            neighborhood: address.neighborhood,
            city: address.city,
            state: address.state,
            user_id: Uuid::from_str(&user_id).unwrap_or_default()
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
    
    pub fn to_domain_service(service_data: ServiceModel) -> Service {
        let travel_cost_cents = CurrencyHelper::to_cents(service_data.travel_cost);
        Service { 
            id: service_data.id.to_string(), 
            name: service_data.name, 
            travel_cost: travel_cost_cents,
            created_at: service_data.created_at.timestamp() 
        }
    }
    
    pub fn to_domain_service_information(service_information_data: ServiceInformationModel) -> ServiceInformation {
        ServiceInformation {
            id: service_information_data.id.to_string(),
            user_id: service_information_data.user_id.to_string(),
            service_id: service_information_data.service_id.to_string(),
            consultant_id: service_information_data.consultant_id
                .map(|i| i.to_string()),
            service_step_id: service_information_data.service_step_id,
            address_id: service_information_data.address_id.to_string(),
            scheduled_at: service_information_data.created_at.timestamp(),
        }
    }
    
    pub fn to_data_service_information(service_information: ServiceInformation) -> ServiceInformationModel {
        ServiceInformationModel {
            id: Uuid::from_str(&service_information.id).unwrap_or_default(),
            user_id: Uuid::from_str(&service_information.user_id).unwrap_or_default(),
            service_id: Uuid::from_str(&service_information.service_id).unwrap_or_default(),
            consultant_id: service_information.consultant_id.map(|ci| Uuid::from_str(&ci).unwrap_or_default()),
            service_step_id: service_information.service_step_id,
            address_id: Uuid::from_str(&service_information.address_id).unwrap_or_default(),
            created_at: DateTime::default(),
        }
    }
    
    pub fn to_data_service_budget(service_budget: ServiceBudget) -> ServiceBudgetModel {
        ServiceBudgetModel {
            id: Uuid::from_str(&service_budget.id).unwrap_or_default(),
            service_information_id: Uuid::from_str(&service_budget.service_information_id).unwrap_or_default(),
            service_cost: Decimal::new(service_budget.service_cost, 2),
            travel_cost: Decimal::new(service_budget.travel_cost, 2),
            description: service_budget.description,
            service_budget_status_id: service_budget.service_budget_status_id,
            created_at: DateTime::from_timestamp(service_budget.created_at, 0)
                .unwrap(),
        }
    }
    
    pub fn to_domain_service_budget(service_budget_data: ServiceBudgetModel) -> ServiceBudget {
        let service_cost_cents = CurrencyHelper::to_cents(service_budget_data.service_cost);
        let travel_cost_cents = CurrencyHelper::to_cents(service_budget_data.travel_cost);
        ServiceBudget {
            id: service_budget_data.id.to_string(),
            service_information_id: service_budget_data.service_information_id.to_string(),
            service_cost: service_cost_cents,
            travel_cost: travel_cost_cents,
            description: service_budget_data.description,
            service_budget_status_id: service_budget_data.service_budget_status_id,
            created_at: service_budget_data.created_at.timestamp(),
        }
    }
}
