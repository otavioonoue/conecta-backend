use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::user::{domain::{entity::address::Address, repository::address_repository::AddressRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::{address_model::AddressModel}}, error::AppError}};

pub struct AddressRepositoryImpl<T: Db> {
    pub db: T
}

impl<T: Db> AddressRepositoryImpl<T> {
    pub fn new(app_state: T) -> Self {
        AddressRepositoryImpl { db: app_state }
    }
}

#[async_trait]
impl AddressRepository for AddressRepositoryImpl<Database<Pool<Postgres>>> {
    async fn create(&self, user_id: String, address: Address) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO addresses (cep, number, street, neighborhood, city, state, user_id)
             VALUES
             ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(address.cep)
        .bind(address.number)
        .bind(address.street)
        .bind(address.neighborhood)
        .bind(address.city)
        .bind(address.state)
        .bind(Uuid::from_str(&user_id).unwrap_or_default())
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        Ok(())
    }

    async fn find_all(&self, user_id: String) -> Result<Vec<Address>, AppError> {
        let resp: Vec<AddressModel> = sqlx::query_as::<_, AddressModel>(
            "SELECT * 
               FROM addresses a
              WHERE a.user_id = $1"
        )
        .bind(Uuid::from_str(&user_id).unwrap_or_default())
        .fetch_all(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.into_iter().map(|um| InfrastructureMapper::to_domain_address(um)).collect())
    }
}