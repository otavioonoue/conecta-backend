use std::sync::Arc;


use crate::{modules::public::consultant::{application::{dto::{add_service_dto::AddServiceDto, create_consultant_dto::CreateConsultantDto, remove_service_dto::RemoveServiceDto}, usecase::UseCase}, domain::{entity::consultant::Consultant, repository::consultant_repository::ConsultantRepository}}, shared::infra::error::AppError};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct ConsultantAppState {
  pub consultant_repository: Arc<dyn ConsultantRepository>,
  pub get_all_consultant: Arc<dyn UseCase<(), Result<Vec<Consultant>, AppError>>>,
  pub create_consultant: Arc<dyn UseCase<CreateConsultantDto, Result<String, AppError>>>,
  pub add_service: Arc<dyn UseCase<(AddServiceDto, String), Result<(), AppError>>>,
  pub remove_service: Arc<dyn UseCase<(RemoveServiceDto, String), Result<(), AppError>>>
}