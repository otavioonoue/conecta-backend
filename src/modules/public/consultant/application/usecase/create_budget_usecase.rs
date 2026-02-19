use async_trait::async_trait;
use http::StatusCode;
use rust_decimal::prelude::ToPrimitive;

use crate::{modules::public::consultant::{application::{dto::create_budget_dto::CreateBudgetDto, usecase::usecase::UseCase}, appstate::ConsultantAppState, domain::entity::service_budget::ServiceBudget}, shared::{domain::entity::notification::Notification, infra::{error::AppError, helpers::currency::CurrencyHelper}}};

pub struct CreateBudgetUseCase;

type Input = (String, CreateBudgetDto);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for CreateBudgetUseCase {
    async fn execute(&self, (service_information_id, budget_dto): Input, s: ConsultantAppState) -> Output {
        let service_information_optional = s.service_repository.find_service_information_by_id(service_information_id).await?;
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        if service_information.service_step_id != 4 {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "The visit hasn't been confirmed yet"))
        }
        
        let Some(service) = s.service_repository.find_service_by_id(service_information.service_id.clone()).await? else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service not found"));
        };

        let budget = ServiceBudget {
            id: String::from(""),
            service_information_id: service_information.id.clone(),
            service_cost: CurrencyHelper::to_cents(budget_dto.service_cost),
            travel_cost: service.travel_cost,
            description: budget_dto.description,
            service_budget_status_id: 1,
            created_at: 0
        };
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id.clone(),
            title: "Orçamento recebido".to_string(),
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.service_repository.create_service_budget(budget).await?;
        
        service_information.service_step_id = 5;
        
        s.service_repository.update_service_information(service_information).await?;
        
        s.notification_service.send(notification, String::from("BUDGET_RECEIVED")).await?;
        
        Ok(())
    }
}