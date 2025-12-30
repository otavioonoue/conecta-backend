use std::str::FromStr;

use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{modules::public::service::{domain::{entity::address::Address, repository::address_repository::AddressRepository}, infrastructure::mapper::InfrastructureMapper}, shared::infra::{database::{db_config::{Database, Db}, model::address_model::AddressModel}, error::AppError}};

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
    async fn find_by_id(&self, address_id: String) -> Result<Option<Address>, AppError> {
        let resp: Option<AddressModel> = sqlx::query_as::<_, AddressModel>(
            "SELECT *
               FROM addresses a
              WHERE a.id = $1"
        )
        .bind(Uuid::from_str(&address_id).unwrap_or_default())
        .fetch_optional(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        return Ok(resp.map(|um| InfrastructureMapper::to_domain_address(um)))
    }
}