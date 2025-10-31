use std::str::FromStr;

use chrono::DateTime;
use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};
use sqlx::types::Uuid;

use crate::{
    modules::public::service::domain::entity::service::Service,
    shared::infra::database::model::service_model::ServiceModel,
};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
    pub fn to_data_consultant(service: Service) -> ServiceModel {
        ServiceModel {
            id: Uuid::from_str(&service.id).unwrap_or_default(),
            name: service.name,
            travel_cost: Decimal::from_i64(service.travel_cost).unwrap_or_default(),
            created_at: DateTime::from_timestamp(service.created_at, 0)
                .unwrap()
                .naive_utc(),
        }
    }
    
    pub fn to_domain_consultant(service_data: ServiceModel) -> Service {
        Service {
            id: service_data.id.to_string(),
            name: service_data.name,
            travel_cost: service_data.travel_cost.to_i64().unwrap_or_default(),
            created_at: service_data.created_at.and_utc().timestamp(),
        }
    }
}
