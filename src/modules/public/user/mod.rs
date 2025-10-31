use std::sync::Arc;

use crate::{modules::public::user::{application::{dto::create_user_dto::CreateUserDto, usecase::UseCase}, domain::{entity::user::User, repository::user_repository::UserRepository}}, shared::infra::error::AppError};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct UserAppState {
  pub user_repository: Arc<dyn UserRepository>,
  pub create_user: Arc<dyn UseCase<CreateUserDto, Result<String, AppError>>>,
  pub get_all_users: Arc<dyn UseCase<(), Result<Vec<User>, AppError>>>
}