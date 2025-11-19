use std::sync::Arc;


use crate::{modules::public::consultant::{application::{dto::{add_service_dto::AddServiceDto, create_consultant_dto::CreateConsultantDto, remove_service_dto::RemoveServiceDto}, usecase::UseCase}, domain::{entity::consultant::Consultant, repository::consultant_repository::ConsultantRepository}}, shared::infra::{error::AppError, service::hash_service::HashService}};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct ConsultantAppState {
  pub consultant_repository: Arc<dyn ConsultantRepository>,
  pub hash_service: Arc<dyn HashService>,
  pub get_all_consultant: Arc<dyn UseCase<(), Result<Vec<Consultant>, AppError>>>,
  pub create_consultant: Arc<dyn UseCase<CreateConsultantDto, Result<String, AppError>>>,
  pub add_service: Arc<dyn UseCase<(AddServiceDto, String), Result<(), AppError>>>,
  pub remove_service: Arc<dyn UseCase<(RemoveServiceDto, String), Result<(), AppError>>>,
  pub find_all_by_service: Arc<dyn UseCase<String, Result<Vec<Consultant>, AppError>>>
}