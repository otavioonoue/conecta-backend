use std::sync::Arc;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{application::{dto::{budget_status_dto::BudgetStatusDto, create_address_dto::CreateAddressDto, create_user_dto::CreateUserDto, update_service_order_status_dto::UpdateServiceOrderStatusDto}, usecase::UseCase}, domain::{entity::{address::Address, user::User}, repository::{address_repository::AddressRepository, service_repository::ServiceRepository, user_repository::UserRepository}}}}, shared::infra::{error::AppError, service::{hash_service::HashService, notification_service::NotificationService}}};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct UserAppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub address_repository: Arc<dyn AddressRepository>,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub hash_service: Arc<dyn HashService>,
    pub notification_service: Arc<dyn NotificationService>,
    pub create_user: Arc<dyn UseCase<CreateUserDto, Result<String, AppError>>>,
    pub get_all_users: Arc<dyn UseCase<(), Result<Vec<User>, AppError>>>,
    pub get_all_addresses: Arc<dyn UseCase<Claims, Result<Vec<Address>, AppError>>>,
    pub create_address: Arc<dyn UseCase<(Claims, CreateAddressDto), Result<(), AppError>>>,
    pub update_budget_status: Arc<dyn UseCase<(Claims, String, BudgetStatusDto), Result<(), AppError>>>,
    pub update_service_order_status: Arc<dyn UseCase<(Claims, String, UpdateServiceOrderStatusDto), Result<(), AppError>>>
}