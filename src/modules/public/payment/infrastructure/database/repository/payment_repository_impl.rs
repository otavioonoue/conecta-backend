use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{modules::public::payment::{domain::{entity::{payment_scheduled::PaymentServiceScheduled}, repository::payment_repository::PaymentRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::{payment_scheduled_model::PaymentServiceScheduledModel}}, error::AppError}};

pub struct PaymentRepositoryImpl<T: Db> {
    pub db: T
}

impl<T: Db> PaymentRepositoryImpl<T> {
    pub fn new(app_state: T) -> Self {
        PaymentRepositoryImpl { db: app_state }
    }
}

#[async_trait]
impl PaymentRepository for PaymentRepositoryImpl<Database<Pool<Postgres>>> {
    async fn create_payment_service_scheduled(&self, payment_service_scheduled: PaymentServiceScheduled) -> Result<(), AppError> {
        let data_payment_service_scheduled = InfrastructureMapper::to_data_payment_service_scheduled(payment_service_scheduled);
        sqlx::query(
            "INSERT INTO payments_service_scheduled (schedule_service_information_id, user_id, provider, provider_payment_id, status, cost) 
             VALUES (
                $1, $2, $3, $4, $5, $6
             )"
        )
        .bind(data_payment_service_scheduled.schedule_service_information_id)
        .bind(data_payment_service_scheduled.user_id)
        .bind(data_payment_service_scheduled.provider)
        .bind(data_payment_service_scheduled.provider_payment_id)
        .bind(data_payment_service_scheduled.status)
        .bind(data_payment_service_scheduled.cost)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
    
    async fn find_by_service_information_id(&self, service_information_id: String) -> Result<Option<PaymentServiceScheduled>, AppError> {
        let resp: Option<PaymentServiceScheduledModel> = sqlx::query_as::<_, PaymentServiceScheduledModel>(
            "SELECT * 
               FROM payments_service_scheduled pss
              WHERE pss.schedule_service_information_id = $1"
        )
        .bind(Uuid::from_str(&service_information_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|pss| InfrastructureMapper::to_domain_payment_service_scheduled(pss)))
    }
    
    async fn find_by_payment_link(&self, payment_link: String) -> Result<Option<PaymentServiceScheduled>, AppError> {
        let resp: Option<PaymentServiceScheduledModel> = sqlx::query_as::<_, PaymentServiceScheduledModel>(
            "SELECT * 
               FROM payments_service_scheduled pss
              WHERE pss.provider_payment_id = $1"
        )
        .bind(payment_link)
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|pss| InfrastructureMapper::to_domain_payment_service_scheduled(pss)))
    }
    
    async fn update_payment(&self, payment_service_scheduled: PaymentServiceScheduled ) -> Result<(), AppError> {
        let data_payment_service_scheduled = InfrastructureMapper::to_data_payment_service_scheduled(payment_service_scheduled);
        sqlx::query(
            "UPDATE payments_service_scheduled pss
                SET status = $1
              WHERE pss.id = $2"
        )
        .bind(data_payment_service_scheduled.status)
        .bind(data_payment_service_scheduled.id)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
}