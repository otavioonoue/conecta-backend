use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

use crate::{modules::public::auth::{domain::{entity::{consultant::Consultant, user::User}, repository::auth_repository::AuthRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::user_model::UserModel}, error::AppError}};
use crate::shared::infra::database::model::consultant_model::ConsultantModel;

pub struct AuthRepositoryImpl<T: Db> {
  pub db: T
}

impl<T: Db> AuthRepositoryImpl<T> {
  pub fn new(app_state: T) -> Self {
    AuthRepositoryImpl { db: app_state }
  }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl<Database<Pool<Postgres>>> {
    async fn find_by_email_consultant(&self, consultant_email: String) -> Result<Option<Consultant>, AppError> {
        let resp = sqlx::query_as::<_, ConsultantModel>(
            "SELECT *
               FROM consultants
              WHERE email = $1
            "
        )
        .bind(consultant_email)
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(resp.map(|cm| InfrastructureMapper::to_domain_consultant(cm)))
    }
    
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError> {
        let resp = sqlx::query_as::<_, UserModel>(
            "SELECT *
               FROM users
              WHERE email = $1
            "
        )
        .bind(email)
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(resp.map(|um| InfrastructureMapper::to_domain_user(um)))
    }
    
}