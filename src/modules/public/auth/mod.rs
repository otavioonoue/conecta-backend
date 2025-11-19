use std::sync::Arc;

use crate::{modules::public::auth::{application::{dto::login_dto::LoginDto, usecase::UseCase}, domain::repository::auth_repository::AuthRepository, presentation::dto::login_response::LoginResponse}, shared::infra::{error::AppError, service::hash_service::HashService}};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct AuthAppState {
    pub auth_repository: Arc<dyn AuthRepository>,
    pub hash_service: Arc<dyn HashService>,
    pub login: Arc<dyn UseCase<LoginDto, Result<LoginResponse, AppError>>>,
    pub login_consultant: Arc<dyn UseCase<LoginDto, Result<LoginResponse, AppError>>>
}