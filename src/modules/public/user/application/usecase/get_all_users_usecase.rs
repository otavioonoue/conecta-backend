use async_trait::async_trait;

use crate::{modules::public::user::{UserAppState, application::usecase::UseCase, domain::{entity::user::User}}, shared::infra::error::AppError};

pub struct GetAllUsersUseCase;

#[async_trait]
impl UseCase<(), Result<Vec<User>, AppError>> for GetAllUsersUseCase {
  async fn execute(&self, _: (), s: UserAppState) -> Result<Vec<User>, AppError> {
    let users = s.user_repository.find_all().await?;
    
    Ok(users)
  }
}