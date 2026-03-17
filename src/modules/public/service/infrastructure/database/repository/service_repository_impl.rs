use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::DateTime;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::service::{domain::{entity::{service::Service, service_information::ServiceInformation, service_schedule::ServiceSchedule}, repository::service_repository::ServiceRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::service_model::ServiceModel}, error::AppError}};

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

    async fn schedule(&self, service_information: ServiceInformation, service_schedule: ServiceSchedule) -> Result<String, AppError> {
        let mut transaction = self.db.pool.begin().await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        let service_information_id: Uuid = sqlx::query_scalar(
            "INSERT INTO service_information (
                user_id,
                service_id,
                consultant_id,
                service_step_id,
                address_id
            ) VALUES (
                $1, $2, $3, $4, $5
            ) RETURNING id"
        )
        .bind(Uuid::from_str(&service_information.user_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_information.service_id).unwrap_or_default())
        .bind(None::<Uuid>)
        .bind(service_information.service_step_id)
        .bind(Uuid::from_str(&service_information.address_id).unwrap_or_default())
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO services_scheduled (
                service_information_id,
               	service_status_id,
                description,
               	scheduled_to
            ) VALUES (
               	$1, $2,	$3, $4
            )"
        )
        .bind(service_information_id)
        .bind(service_schedule.service_status_id)
        .bind(service_schedule.description)
        .bind(DateTime::from_timestamp_secs(service_schedule.scheduled_to))
        .execute(&mut *transaction)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        transaction.commit().await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(service_information_id.to_string())
    }
    
    async fn find_by_user(&self, _: String) -> Result<(), AppError> {
        let _: Vec<ServiceModel> = sqlx::query_as::<_, ServiceModel>(
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