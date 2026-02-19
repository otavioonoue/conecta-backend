use std::sync::Arc;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, consultant::{application::{dto::{add_service_dto::AddServiceDto, create_budget_dto::CreateBudgetDto, create_consultant_dto::CreateConsultantDto, remove_service_dto::RemoveServiceDto}, usecase::usecase::UseCase}, domain::{entity::consultant::Consultant, repository::{consultant_repository::ConsultantRepository, service_repository::ServiceRepository}}}}, shared::infra::{error::AppError, service::{hash_service::HashService, notification_service::NotificationService}}};

#[derive(Clone)]
pub struct ConsultantAppState {
    pub consultant_repository: Arc<dyn ConsultantRepository>,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub notification_service: Arc<dyn NotificationService>,
    pub hash_service: Arc<dyn HashService>,
    pub get_all_consultant: Arc<dyn UseCase<(), Result<Vec<Consultant>, AppError>>>,
    pub create_consultant: Arc<dyn UseCase<CreateConsultantDto, Result<String, AppError>>>,
    pub add_service: Arc<dyn UseCase<(AddServiceDto, String), Result<(), AppError>>>,
    pub remove_service: Arc<dyn UseCase<(RemoveServiceDto, String), Result<(), AppError>>>,
    pub find_all_by_service: Arc<dyn UseCase<String, Result<Vec<Consultant>, AppError>>>,
    pub confirm_scheduled_service: Arc<dyn UseCase<(Claims, String), Result<(), AppError>>>,
    pub create_service_budget: Arc<dyn UseCase<(String, CreateBudgetDto), Result<(), AppError>>>
}