use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use rand::seq::IndexedRandom;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, service::{application::{dto::schedule_service_dto::{ScheduleServiceDto, ScheduleServiceResponse}, usecase::UseCase}, appstate::ServiceAppState, domain::entity::{service_information::ServiceInformation, service_schedule::ServiceSchedule}}}, shared::infra::error::AppError};

pub struct ScheduleServiceUseCase;

type Input = (Claims, ScheduleServiceDto);
type Output = Result<ScheduleServiceResponse, AppError>;

#[async_trait]
impl UseCase<Input, Output> for ScheduleServiceUseCase {
    async fn execute(&self, (user, dto): (Claims, ScheduleServiceDto), s: ServiceAppState) -> Output {
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
        
        // // Select a random provider
        // let selected_consultant = {
        //     let mut rng = rand::rng(); 
        //     consultants.choose(&mut rng).unwrap()
        // };
        
        let service_information = ServiceInformation::new(
            user.sub, 
            service.id, 
            String::from(""), 
            2, 
            address.id
        );  
        
        let schedule = ServiceSchedule::new(
            service_information.id.clone(), 
            1, 
            dto.description.unwrap_or_default(), 
            dto.schedule_to
        );
        
        let schedule_service_id = s.service_repository.schedule(service_information, schedule).await?;
      
        let response = ScheduleServiceResponse {
            service_information_id: schedule_service_id
        };
        
        Ok(response)
    }
}