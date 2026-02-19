use std::str::FromStr;

use async_trait::async_trait;
use http::StatusCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{modules::public::user::{domain::{entity::{service::Service, service_budget::ServiceBudget, service_information::ServiceInformation}, repository::service_repository::ServiceRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::{service_budget_model::ServiceBudgetModel, service_information_model::ServiceInformationModel, service_model::ServiceModel}}, error::AppError}};

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
    
    async fn find_service_information_by_service_budget_id(&self, service_budget_id: String) -> Result<Option<ServiceInformation>, AppError> {
        let resp: Option<ServiceInformationModel> = sqlx::query_as::<_, ServiceInformationModel>(
            "SELECT si.* 
               FROM service_information si
              INNER JOIN services_budgets sb 
                 ON sb.service_information_id = si.id
              WHERE sb.id = $1"
        )
        .bind(Uuid::from_str(&service_budget_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|si| InfrastructureMapper::to_domain_service_information(si)))
    }
    
    async fn update_service_information(&self, service_information: ServiceInformation) -> Result<(), AppError> {
        let data_service_information = InfrastructureMapper::to_data_service_information(service_information);
        sqlx::query(
            "UPDATE service_information si
                SET service_step_id = $1,
                    consultant_id = $2
              WHERE si.id = $3"
        )
        .bind(data_service_information.service_step_id)
        .bind(data_service_information.consultant_id)
        .bind(data_service_information.id)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
    
    async fn create_service_budget(&self, service_budget: ServiceBudget) -> Result<(), AppError> {
        let data_service_budget = InfrastructureMapper::to_data_service_budget(service_budget);
        sqlx::query(
            "INSERT INTO services_budgets (
                service_information_id, service_cost, travel_cost, description, service_budget_status_id
            )
            VALUES (
                $1, $2, $3, $4, $5
            )"
        )
        .bind(data_service_budget.service_information_id)
        .bind(data_service_budget.service_cost)
        .bind(data_service_budget.travel_cost)
        .bind(data_service_budget.description)
        .bind(data_service_budget.service_budget_status_id)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
    
    async fn find_service_budget_by_id(&self, service_budget_id: String) -> Result<Option<ServiceBudget>, AppError> {
        let service_budget_optional = sqlx::query_as::<_, ServiceBudgetModel>(
            "SELECT * FROM services_budgets sb
              WHERE sb.id = $1"
        )
        .bind(Uuid::from_str(&service_budget_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(service_budget_optional
            .map(|sbm| InfrastructureMapper::to_domain_service_budget(sbm))
        )
    }
    
    async fn update_service_budget_status(&self, service_budget: ServiceBudget) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE services_budgets sb
              SET service_budget_status_id = $1
              WHERE sb.id = $2"
        )
        .bind(service_budget.service_budget_status_id)
        .bind(Uuid::from_str(&service_budget.id).unwrap_or_default())
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(())
    }
}