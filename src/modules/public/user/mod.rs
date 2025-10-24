use sqlx::{Pool, Postgres};

use crate::{modules::public::user::{application::usecase::{create_user_usecase::CreateUserUseCase, get_all_users_usecase::GetAllUsersUseCase}, infrastructure::database::repository::user_repository_impl::UserRepositoryImpl}, shared::infra::database::db_config::Database};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub struct UserAppState {
  pub user_repository: UserRepositoryImpl<Database<Pool<Postgres>>>,
  pub create_user: CreateUserUseCase,
  pub get_all_users: GetAllUsersUseCase
}