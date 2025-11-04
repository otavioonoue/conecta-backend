use std::str::FromStr;
use rust_decimal::prelude::ToPrimitive;
use chrono::DateTime;
use sqlx::types::Uuid;

use crate::{
    modules::public::consultant::domain::entity::{consultant::Consultant, service::Service},
    shared::infra::database::model::{consultant_model::ConsultantModel, service_model::ServiceModel},
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
    
    pub fn to_domain_service(service_data: ServiceModel) -> Service {
        Service { 
            id: service_data.id.to_string(), 
            name: service_data.name, 
            travel_cost: service_data.travel_cost.to_i64().unwrap_or_default(), 
            created_at: service_data.created_at.and_utc().timestamp() 
        }
    }
}
