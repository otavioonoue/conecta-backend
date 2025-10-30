use crate::{modules::public::user::domain::{entity::user::User, repository::user_repository::UserRepository, usecase::UseCase}, shared::infra::error::AppError};

pub struct GetAllUsersUseCase;

impl UseCase<(), Result<Vec<User>, AppError>> for GetAllUsersUseCase {
    async fn execute(&self, _: (), s: std::sync::Arc<crate::modules::public::user::UserAppState>) -> Result<Vec<User>, AppError> {
        let users = s.user_repository.find_all().await?;
        
        Ok(users)
    }
}