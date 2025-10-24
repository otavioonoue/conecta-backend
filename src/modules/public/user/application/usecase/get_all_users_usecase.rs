use std::error::Error;

use crate::modules::public::user::domain::{entity::user::User, repository::user_repository::UserRepository, usecase::UseCase};

pub struct GetAllUsersUseCase;

impl UseCase<(), Result<Vec<User>, Box<dyn Error>>> for GetAllUsersUseCase {
    async fn execute(&self, _: (), s: std::sync::Arc<crate::modules::public::user::UserAppState>) -> Result<Vec<User>, Box<dyn Error>> {
        let users = s.user_repository.find_all().await?;
        
        Ok(users)
    }
}