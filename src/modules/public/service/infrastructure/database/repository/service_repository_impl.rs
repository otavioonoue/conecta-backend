use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::DateTime;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::service::{domain::{entity::{service::Service, service_schedule::ServiceSchedule}, repository::service_repository::ServiceRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::service_model::ServiceModel}, error::AppError}};

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
    async fn create(&self, service: Service) -> Result<(), AppError> {
        let service_data = InfrastructureMapper::to_data_service(service);
        
        let _: Uuid = sqlx::query_scalar(
            "INSERT INTO services (
                name,
                travel_cost
            ) VALUES
            (
                $1, $2
            ) RETURNING id"
        )
        .bind(service_data.name)
        .bind(service_data.travel_cost)
        .fetch_one(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Service>, AppError> {
        let resp: Vec<ServiceModel> = sqlx::query_as::<_, ServiceModel>(
            "SELECT * FROM services"
        )
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.into_iter().map(|sm| InfrastructureMapper::to_domain_service(sm)).collect())
    }
    
    async fn find_by_id(&self, service_id: String) -> Result<Option<Service>, AppError> {
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

    async fn schedule(&self, service_schedule: ServiceSchedule) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO services_scheduled (
               	service_id,
               	user_id,
               	consultant_id,
               	service_status_id,
                description,
                address_id,
               	scheduled_to
            ) VALUES (
               	$1, $2,	$3,	$4,	$5, $6, $7
            )"
        )
        .bind(Uuid::from_str(&service_schedule.service_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_schedule.user_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_schedule.consultant_id).unwrap_or_default())
        .bind(service_schedule.service_status_id)
        .bind(service_schedule.description)
        .bind(Uuid::from_str(&service_schedule.address_id).unwrap_or_default())
        .bind(DateTime::from_timestamp_secs(service_schedule.scheduled_to))
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(())
    }
    
    async fn find_by_user(&self, user_id: String) -> Result<(), AppError> {
        let resp: Vec<ServiceModel> = sqlx::query_as::<_, ServiceModel>(
            "SELECT ss.id        as scheduled_id,
                    s.name       as name,
                    s.travel_cost,
                    a.id         as address_id,
                    a.cep,
                    a.number,
                    a.street,
                    a.neighborhood,
                    a.city,
                    a.state,
                    a.user_id
               FROM services_scheduled ss
               JOIN services s ON ss.service_id = s.id
               JOIN addresses a ON ss.address_id = a.id
              WHERE ss.user_id = $1"
        )
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(())
    }
}