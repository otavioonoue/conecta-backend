use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::auth::{domain::{entity::user::User, repository::auth_repository::AuthRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::user_model::UserModel}, error::AppError}};


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
    // async fn find_by_id(&self, consultant_id: String) -> Result<Option<User>, AppError> {
    //     let resp = sqlx::query_as::<_, ConsultantModel>(
    //         "SELECT *
    //            FROM consultants
    //           WHERE id = $1
    //         "
    //     )
    //     .bind(Uuid::from_str(&consultant_id).unwrap_or_default())
    //     .fetch_optional(&*self.db.pool)
    //     .await
    //     .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    //     Ok(resp.map(|cm| InfrastructureMapper::to_domain_consultant(cm)))
    // }
    
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