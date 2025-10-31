use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::service::{domain::{entity::service::Service, repository::service_repository::ServiceRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::service_model::ServiceModel}, error::AppError}};

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
        let service_data = InfrastructureMapper::to_data_consultant(service);
        
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
        
        return Ok(resp.into_iter().map(|sm| InfrastructureMapper::to_domain_consultant(sm)).collect())
    }
}