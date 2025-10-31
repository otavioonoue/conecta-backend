use std::sync::Arc;

use crate::{modules::public::service::{application::{dto::create_service_dto::{CreateServiceDto}, usecase::UseCase}, domain::{entity::service::Service, repository::service_repository::ServiceRepository}}, shared::infra::error::AppError};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct ServiceAppState {
  pub service_repository: Arc<dyn ServiceRepository>,
  pub get_all_services: Arc<dyn UseCase<(), Result<Vec<Service>, AppError>>>,
  pub create_service: Arc<dyn UseCase<CreateServiceDto, Result<String, AppError>>>
}