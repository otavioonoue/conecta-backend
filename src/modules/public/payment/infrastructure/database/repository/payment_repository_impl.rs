use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{modules::public::payment::{domain::{entity::service_payment::ServicePayment, repository::payment_repository::PaymentRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::service_payment_model::{PaymentKindModel, ServicePaymentModel}}, error::AppError}};

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
    async fn create_service_payment(&self, service_payment: ServicePayment) -> Result<(), AppError> {
        let data_service_payment = InfrastructureMapper::to_data_service_payment(service_payment);
        sqlx::query(
            "INSERT INTO service_payments (schedule_service_information_id, user_id, provider, provider_payment_id, kind, status, cost) 
             VALUES (
                $1, $2, $3, $4, $5, $6, $7
             )"
        )
        .bind(data_service_payment.schedule_service_information_id)
        .bind(data_service_payment.user_id)
        .bind(data_service_payment.provider)
        .bind(data_service_payment.provider_payment_id)
        .bind(data_service_payment.kind)
        .bind(data_service_payment.status)
        .bind(data_service_payment.cost)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
    
    async fn find_by_service_information_id(&self, service_information_id: String, kind: String) -> Result<Option<ServicePayment>, AppError> {
        let resp: Option<ServicePaymentModel> = sqlx::query_as::<_, ServicePaymentModel>(
            "SELECT * 
               FROM service_payments sp
              WHERE sp.schedule_service_information_id = $1
                AND sp.kind = $2"
        )
        .bind(Uuid::from_str(&service_information_id).unwrap_or_default())
        .bind(PaymentKindModel::from_str(&kind)?)
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|spm| InfrastructureMapper::to_domain_service_payment(spm)))
    }
    
    async fn find_by_payment_link(&self, payment_link: String) -> Result<Option<ServicePayment>, AppError> {
        let resp: Option<ServicePaymentModel> = sqlx::query_as::<_, ServicePaymentModel>(
            "SELECT * 
               FROM service_payments sp
              WHERE sp.provider_payment_id = $1"
        )
        .bind(payment_link)
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(resp.map(|spm| InfrastructureMapper::to_domain_service_payment(spm)))
    }
    
    async fn update_payment(&self, service_payment: ServicePayment) -> Result<(), AppError> {
        let data_service_payment = InfrastructureMapper::to_data_service_payment(service_payment);
        sqlx::query(
            "UPDATE service_payments sp
                SET status = $1
              WHERE sp.id = $2"
        )
        .bind(data_service_payment.status)
        .bind(data_service_payment.id)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(())
    }
}