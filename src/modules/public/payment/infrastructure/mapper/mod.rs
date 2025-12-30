use std::str::FromStr;
use chrono::DateTime;
use rust_decimal::Decimal;
use sqlx::types::Uuid;

use crate::{
    modules::public::{auth::domain::entity::{consultant::Consultant, user::User}, payment::domain::entity::service::Service},
    shared::infra::{database::model::{consultant_model::ConsultantModel, service_model::ServiceModel, user_model::UserModel}, helpers::currency::CurrencyHelper},
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
                .unwrap()
                .naive_utc(),
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
            created_at: user_data.created_at.and_utc().timestamp(),
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
}
