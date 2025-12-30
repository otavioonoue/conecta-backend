use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use rand::seq::IndexedRandom;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, service::{application::{dto::schedule_service_dto::ScheduleServiceDto, usecase::UseCase}, appstate::ServiceAppState, domain::entity::service_schedule::ServiceSchedule}}, shared::infra::error::AppError};

pub struct ScheduleServiceUseCase;

#[async_trait]
impl UseCase<(Claims, ScheduleServiceDto), Result<(), AppError>> for ScheduleServiceUseCase {
    async fn execute(&self, (user, dto): (Claims, ScheduleServiceDto), s: ServiceAppState) -> Result<(), AppError> {
        let option_service = s.service_repository.find_by_id(dto.service_id.clone()).await?;
      
        let Some(service) = option_service else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service not found"));
        };
        
        let option_address = s.address_repository.find_by_id(dto.address_id.clone()).await?;
        
        let Some(address) = option_address else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Address not found"));
        };
        
        let now = Utc::now();

        if dto.schedule_to < now.timestamp() {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Scheduled date cannot be in the past"))
        }
        
        let max_date = now + Duration::days(30);
        
        if dto.schedule_to > max_date.timestamp() {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Scheduled date cannot exceed 30 days from now"))
        }
        
        let consultants = s.consultant_repository.find_all_by_service(dto.service_id.clone()).await?;
        
        if consultants.len() == 0 {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Not found consultants for service"))
        }
        
        let selected_consultant = {
            let mut rng = rand::rng(); 
            consultants.choose(&mut rng).unwrap()
        };
        
        let schedule = ServiceSchedule::new(
            service.id, 
            user.sub, 
            selected_consultant.id.clone(), 
            1, 
            dto.description.unwrap_or_default(),
            address.id,
            dto.schedule_to
        );
        
        let schedule_service = s.service_repository.schedule(schedule).await?;
      
        Ok(schedule_service)
    }
}