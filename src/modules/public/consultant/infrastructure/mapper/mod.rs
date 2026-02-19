use std::str::FromStr;
use rust_decimal::Decimal;
use chrono::DateTime;
use sqlx::types::Uuid;

use crate::{
    modules::public::consultant::domain::entity::{consultant::Consultant, service::Service, service_budget::ServiceBudget, service_information::ServiceInformation},
    shared::infra::{database::model::{consultant_model::ConsultantModel, service_budget_model::ServiceBudgetModel, service_information_model::ServiceInformationModel, service_model::ServiceModel}, helpers::currency::CurrencyHelper},
};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
    pub fn to_data_consultant(consultant: Consultant) -> ConsultantModel {
        ConsultantModel {
            id: Uuid::from_str(&consultant.id).unwrap_or_default(),
            name: consultant.name,
            email: consultant.email,
            phone: consultant.phone,
            password: consultant.password,
            active: consultant.active,
            created_at: DateTime::from_timestamp(consultant.created_at, 0)
                .unwrap(),
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
            created_at: consultant_data.created_at.timestamp(),
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
