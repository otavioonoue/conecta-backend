use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{UserAppState, application::{dto::budget_status_dto::{BudgetStatusDto, Status}, usecase::UseCase}}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct UpdateBudgetStatusUseCase;

type Input = (Claims, String, BudgetStatusDto);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for UpdateBudgetStatusUseCase {
    async fn execute(&self, (claims, service_budget_id, budget_dto): Input, s: UserAppState) -> Output {
        let Some(mut service_budget) = s.service_repository.find_service_budget_by_id(service_budget_id.clone()).await? else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service budget not found"));
        };
        
        let service_information_optional = s.service_repository.find_service_information_by_service_budget_id(service_budget_id).await?;
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        if service_information.service_step_id != 5 {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "The visit hasn't been confirmed yet"))
        }
        
        if let Status::APPROVED = budget_dto.status {
            service_budget.service_budget_status_id = 2;
            service_information.service_step_id = 6;
        } else {
            service_budget.service_budget_status_id = 3;
            service_information.service_step_id = 7;
        }
        
        s.service_repository.update_service_budget_status(service_budget).await?;
        
        s.service_repository.update_service_information(service_information.clone()).await?;
        
        let mut title_notification = String::from("");
        let mut status_notification = String::from("");
    
        match budget_dto.status {
            Status::APPROVED => {
                title_notification = String::from("Orçamento Aceito");
                status_notification = String::from("BUDGET_ACCEPTED");
            },
            Status::REJECTED => {
                title_notification = String::from("Orçamento Negado");
                status_notification = String::from("BUDGET_DENIED");
            }
        }
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id,
            title: title_notification,
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.notification_service.send(notification, status_notification).await?;
        Ok(())
    }
}