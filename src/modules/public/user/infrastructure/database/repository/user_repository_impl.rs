use std::error::Error;

use sqlx::{postgres::PgQueryResult, Pool, Postgres};

use crate::{modules::public::user::{domain::{entity::user::User, repository::user_repository::UserRepository}, infrastructure::{exceptions::InfrastructureException, mapper::InfrastructureMapper}}, shared::infra::database::{db_config::Database, model::user_model::UserModel}};

pub struct UserRepositoryImpl<T> {
    pub db: T
}

impl<T> UserRepositoryImpl<T> {
    pub fn new(app_state: T) -> Self {
        UserRepositoryImpl { db: app_state }
    }
}

impl UserRepository<PgQueryResult> for UserRepositoryImpl<Database<Pool<Postgres>>> {
    async fn create(&self, user: User) -> Result<PgQueryResult, Box<dyn Error>> {
        let user_data = InfrastructureMapper::to_data(user);
        let resp = sqlx::query(
            "INSERT INTO users (
                name, 
                email, 
                phone, 
                cpf, 
                address_id,
                password) VALUES
            (
                $1, $2, $3, $4, $5, $6
            )"
        )
        .bind(user_data.name)
        .bind(user_data.email)
        .bind(user_data.phone)
        .bind(user_data.cpf)
        .bind(user_data.address_id)
        .bind(user_data.password)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| InfrastructureException::InternalDatabaseError(e.to_string()))?;
        
        Ok(resp)
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let resp: Vec<UserModel> = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users"
        )
        .fetch_all(&*self.db.pool)
        .await?;
        
        return Ok(resp.into_iter().map(|um| InfrastructureMapper::to_domain(um)).collect())
    }
}