use std::{error::Error, sync::Arc};

use crate::modules::public::user::{application::{dto::create_user_dto::CreateUserDto, mapper::ApplicationMapper}, domain::{repository::user_repository::UserRepository, usecase::UseCase}, UserAppState};

pub struct CreateUserUseCase;

impl UseCase<CreateUserDto, Result<String, Box<dyn Error>>> for CreateUserUseCase {
    async fn execute(&self, input: CreateUserDto, s: Arc<UserAppState>) -> Result<String, Box<dyn Error>> {
        let user = ApplicationMapper::to_domain(input);
        s.user_repository.create(user).await?;
        Ok(String::from("Created!"))
    }
}