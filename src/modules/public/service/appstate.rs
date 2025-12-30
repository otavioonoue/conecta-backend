use std::sync::Arc;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, service::{application::{dto::{create_service_dto::CreateServiceDto, schedule_service_dto::ScheduleServiceDto, service_dto::ServiceDto}, usecase::UseCase}, domain::{repository::{address_repository::AddressRepository, consultant_repository::ConsultantRepository, service_repository::ServiceRepository}}}}, shared::infra::error::AppError};

#[derive(Clone)]
pub struct ServiceAppState {
    pub service_repository: Arc<dyn ServiceRepository>,
    pub consultant_repository: Arc<dyn ConsultantRepository>,
    pub address_repository: Arc<dyn AddressRepository>,
    pub get_all_services: Arc<dyn UseCase<(), Result<Vec<ServiceDto>, AppError>>>,
    pub create_service: Arc<dyn UseCase<CreateServiceDto, Result<String, AppError>>>,
    pub schedule_service: Arc<dyn UseCase<(Claims, ScheduleServiceDto), Result<(), AppError>>>
}