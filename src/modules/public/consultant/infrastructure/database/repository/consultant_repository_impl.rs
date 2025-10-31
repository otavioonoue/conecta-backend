use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::consultant::{domain::{entity::consultant::Consultant, repository::consultant_repository::ConsultantRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::consultant_model::ConsultantModel}, error::AppError}};

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
}