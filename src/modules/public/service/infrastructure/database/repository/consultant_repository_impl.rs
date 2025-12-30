use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::service::{domain::{entity::{consultant::Consultant, service::Service}, repository::consultant_repository::ConsultantRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::{consultant_model::ConsultantModel, service_model::ServiceModel}}, error::AppError}};

pub struct ConsultantRepositoryImpl<T: Db> {
  pub db: T
}

impl<T: Db> ConsultantRepositoryImpl<T> {
  pub fn new(app_state: T) -> Self {
    ConsultantRepositoryImpl { db: app_state }
  }
}

#[async_trait]
impl ConsultantRepository for ConsultantRepositoryImpl<Database<Pool<Postgres>>> {
    async fn create(&self, consultant: Consultant) -> Result<(), AppError> {
        let consultant_data = InfrastructureMapper::to_data_consultant(consultant);
        
        let _: Uuid = sqlx::query_scalar(
            "INSERT INTO consultants (
                name,
                email,
                phone,
                password
            ) VALUES
            (
                $1, $2, $3, $4
            ) RETURNING id"
        )
        .bind(consultant_data.name)
        .bind(consultant_data.email)
        .bind(consultant_data.phone)
        .bind(consultant_data.password)
        .fetch_one(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Consultant>, AppError> {
        let resp: Vec<ConsultantModel> = sqlx::query_as::<_, ConsultantModel>(
            "SELECT * FROM consultants"
        )
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.into_iter().map(|cm| InfrastructureMapper::to_domain_consultant(cm)).collect())
    }
    
    async fn find_all_by_service(&self, service_id: String) -> Result<Vec<Consultant>, AppError> {
        let resp: Vec<ConsultantModel> = sqlx::query_as::<_, ConsultantModel>(
            "SELECT c.id,
                    c.name,
                    c.email,
                    c.phone,
                    c.password,
                    c.active,
                    c.created_at 
               FROM consultants c
              INNER JOIN services_consultants sc
                 ON c.id = sc.consultant_id
                AND sc.service_id = $1"
        )
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.into_iter().map(|cm| InfrastructureMapper::to_domain_consultant(cm)).collect())
    }
    
    
    async fn find_by_id(&self, consultant_id: String) -> Result<Option<Consultant>, AppError> {
        let resp = sqlx::query_as::<_, ConsultantModel>(
            "SELECT *
               FROM consultants
              WHERE id = $1
            "
        )
        .bind(Uuid::from_str(&consultant_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(resp.map(|cm| InfrastructureMapper::to_domain_consultant(cm)))
    }
    
    async fn find_service_by_id(&self, service_id: String) -> Result<Option<Service>, AppError> {
        let resp = sqlx::query_as::<_, ServiceModel>(
            "SELECT *
               FROM services
              WHERE id = $1
            "
        )
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(resp.map(|sm| InfrastructureMapper::to_domain_service(sm)))
    }
    
    async fn consultant_has_service(&self, consultant_id: String, service_id: String) -> Result<bool, AppError> {
        let resp: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1
                  FROM services_consultants
                 WHERE consultant_id = $1
                   AND service_id = $2
            )
            "
        )
        .bind(Uuid::from_str(&consultant_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .fetch_one(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't verify services_consultants: {}", e.to_string())))?;
        Ok(resp)
    }
    
    async fn add_service(&self, consultant_id: String, service_id: String) -> Result<(), AppError> {
        let _ = sqlx::query(
            "INSERT INTO services_consultants (
                consultant_id,
                service_id
            ) VALUES (
                $1, $2
            )
        ")
        .bind(Uuid::from_str(&consultant_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't create services_consultants: {}", e.to_string())))?;
        Ok(())
    }
    
    async fn remove_service(&self, consultant_id: String, service_id: String) -> Result<(), AppError> {
        let _ = sqlx::query(
            "DELETE FROM services_consultants
              WHERE consultant_id = $1
                AND service_id = $2
            "
        )
        .bind(Uuid::from_str(&consultant_id).unwrap_or_default())
        .bind(Uuid::from_str(&service_id).unwrap_or_default())
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't delete services_consultants: {}", e.to_string())))?;
        
        Ok(())
    }
}