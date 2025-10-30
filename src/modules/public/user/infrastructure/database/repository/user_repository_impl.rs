use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::user::{domain::{entity::{address::Address, user::User}, repository::user_repository::UserRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::Database, model::user_model::UserModel}, error::AppError}};

pub struct UserRepositoryImpl<T> {
    pub db: T
}

impl<T> UserRepositoryImpl<T> {
    pub fn new(app_state: T) -> Self {
        UserRepositoryImpl { db: app_state }
    }
}

impl UserRepository for UserRepositoryImpl<Database<Pool<Postgres>>> {
    async fn create(&self, user: User, address: Address) -> Result<(), AppError> {
        let user_data = InfrastructureMapper::to_data_user(user);
        
        let mut transaction = self.db.pool.begin().await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        let user_id: Uuid = sqlx::query_scalar(
            "INSERT INTO users (
                name, 
                email, 
                phone, 
                cpf,
                password
            ) VALUES
            (
                $1, $2, $3, $4, $5
            ) RETURNING id"
        )
        .bind(user_data.name)
        .bind(user_data.email)
        .bind(user_data.phone)
        .bind(user_data.cpf)
        .bind(user_data.password)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        let address_data = InfrastructureMapper::to_data_address(address, user_id.to_string());
        
        sqlx::query(
            "INSERT INTO addresses (
                cep,
                number,
                street,
                neighborhood,
                city,
                state,
                user_id
            ) VALUES
            (
                $1, $2, $3, $4, $5, $6, $7
            )"
        )
        .bind(address_data.cep)
        .bind(address_data.number)
        .bind(address_data.street)
        .bind(address_data.neighborhood)
        .bind(address_data.city)
        .bind(address_data.state)
        .bind(address_data.user_id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        transaction.commit().await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let resp: Vec<UserModel> = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users"
        )
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.into_iter().map(|um| InfrastructureMapper::to_domain_user(um)).collect())
    }
}