use std::str::FromStr;

use async_trait::async_trait;
use http::StatusCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{modules::public::payment::{domain::{entity::{service::Service, service_information::ServiceInformation}, repository::service_repository::ServiceRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::{service_information_model::ServiceInformationModel, service_model::ServiceModel}}, error::AppError}};

pub struct ServiceRepositoryImpl<T: Db> {
    pub db: T
}

impl<T: Db> ServiceRepositoryImpl<T> {
    pub fn new(app_state: T) -> Self {
        ServiceRepositoryImpl { db: app_state }
    }
}

#[async_trait]
impl ServiceRepository for ServiceRepositoryImpl<Database<Pool<Postgres>>> {
    async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError> {
        let resp: Option<ServiceModel> = sqlx::query_as::<_, ServiceModel>(
            "SELECT * 
               FROM services s
              WHERE s.id = $1"
        )
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|s| InfrastructureMapper::to_domain_service(s)))
    }
    
    async fn find_service_information_by_id(&self, service_information_id: String) -> Result<Option<ServiceInformation>, AppError> {
        let resp: Option<ServiceInformationModel> = sqlx::query_as::<_, ServiceInformationModel>(
            "SELECT * 
               FROM service_information si
              WHERE si.id = $1"
        )
        .bind(Uuid::from_str(&service_information_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|si| InfrastructureMapper::to_domain_service_information(si)))
    }
    
    async fn update_service_information(&self, service_information: ServiceInformation) -> Result<(), AppError> {
        let data_service_information = InfrastructureMapper::to_data_service_information(service_information);
        sqlx::query(
            "UPDATE service_information si
                SET service_step_id = $1
              WHERE si.id = $2"
        )
        .bind(data_service_information.service_step_id)
        .bind(data_service_information.id)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
}