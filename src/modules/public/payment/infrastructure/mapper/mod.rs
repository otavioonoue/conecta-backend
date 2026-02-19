use std::str::FromStr;
use chrono::DateTime;
use rust_decimal::Decimal;
use sqlx::types::Uuid;

use crate::{
    modules::public::{auth::domain::entity::{consultant::Consultant, user::User}, payment::domain::entity::{payment_scheduled::PaymentServiceScheduled, service::Service, service_information::ServiceInformation}},
    shared::infra::{database::model::{consultant_model::ConsultantModel, payment_scheduled_model::PaymentServiceScheduledModel, service_information_model::ServiceInformationModel, service_model::ServiceModel, user_model::UserModel}, helpers::currency::CurrencyHelper},
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
    
    pub fn to_data_service(service: Service) -> ServiceModel {
        ServiceModel {
            id: Uuid::from_str(&service.id).unwrap_or_default(),
            name: service.name,
            travel_cost: Decimal::new(service.travel_cost, 2),
            created_at: DateTime::from_timestamp(service.created_at, 0)
                .unwrap(),
        }
    }
    
    pub fn to_domain_service(service_data: ServiceModel) -> Service {
		let travel_cost_cents = CurrencyHelper::to_cents(service_data.travel_cost);
        Service {
            id: service_data.id.to_string(),
            name: service_data.name,
            travel_cost: travel_cost_cents,
            created_at: service_data.created_at.timestamp(),
        }
    }
    
    pub fn to_domain_service_information(service_information_data: ServiceInformationModel) -> ServiceInformation {
        ServiceInformation {
            id: service_information_data.id.to_string(),
            user_id: service_information_data.user_id.to_string(),
            service_id: service_information_data.service_id.to_string(),
            consultant_id: service_information_data.consultant_id
                .map(|ci| ci.to_string()),
            service_step_id: service_information_data.service_step_id,
            address_id: service_information_data.address_id.to_string(),
            scheduled_at: service_information_data.created_at.timestamp(),
        }
    }
    
    pub fn to_domain_payment_service_scheduled(pssm: PaymentServiceScheduledModel) -> PaymentServiceScheduled {
        let cost_cents = CurrencyHelper::to_cents(pssm.cost);
        PaymentServiceScheduled {
            id: pssm.id.to_string(),
            schedule_service_information_id: pssm.schedule_service_information_id.to_string(),
            user_id: pssm.user_id.to_string(),
            provider: pssm.provider,
            provider_payment_id: pssm.provider_payment_id,
            status: pssm.status,
            cost: cost_cents,
            created_at: pssm.created_at.timestamp(),
        }
    }
    
    pub fn to_data_payment_service_scheduled(pss: PaymentServiceScheduled) -> PaymentServiceScheduledModel {
        PaymentServiceScheduledModel {
            id: Uuid::from_str(&pss.id).unwrap_or_default(),
            schedule_service_information_id: Uuid::from_str(&pss.schedule_service_information_id).unwrap_or_default(),
            user_id: Uuid::from_str(&pss.user_id).unwrap_or_default(),
            provider: pss.provider,
            provider_payment_id: pss.provider_payment_id,
            status: pss.status,
            cost: Decimal::new(pss.cost, 2),
            created_at: DateTime::from_timestamp(pss.created_at, 0)
                .unwrap(),
        }
    }
    
    pub fn to_data_service_information(service_information: ServiceInformation) -> ServiceInformationModel {
        ServiceInformationModel {
            id: Uuid::from_str(&service_information.id).unwrap_or_default(),
            user_id: Uuid::from_str(&service_information.user_id).unwrap_or_default(),
            service_id: Uuid::from_str(&service_information.service_id).unwrap_or_default(),
            consultant_id: service_information.consultant_id
                .map(|ci| Uuid::from_str(&ci).unwrap_or_default()),
            service_step_id: service_information.service_step_id,
            address_id: Uuid::from_str(&service_information.address_id).unwrap_or_default(),
            created_at: DateTime::default(),
        }
    }
}
