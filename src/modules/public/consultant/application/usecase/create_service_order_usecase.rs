use async_trait::async_trait;
use http::StatusCode;
use rust_decimal::Decimal;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, consultant::{application::{dto::create_service_order_dto::CreateServiceOrderDto, usecase::usecase::UseCase}, appstate::ConsultantAppState, domain::entity::service_order::ServiceOrder}}, shared::{domain::entity::notification::Notification, infra::{error::AppError, helpers::currency::CurrencyHelper}}};

pub struct CreateServiceOrderUseCase;

type Input = (Claims, String, CreateServiceOrderDto);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for CreateServiceOrderUseCase {
    async fn execute(&self, (claims, service_information_id, dto): Input, s: ConsultantAppState) -> Output {
        let service_information_optional = s.service_repository.find_service_information_by_id(service_information_id.clone()).await?;
        
        if let Some(service_information) = service_information_optional.as_ref() {
            if service_information.service_step_id != 10 {
                return Err(AppError::new(StatusCode::BAD_REQUEST, "The budget payment hasn't been received yet"))
            }
        
            if claims.sub != *service_information.consultant_id.as_ref().unwrap() {
                return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
            }
        }
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        let services_budgets = s.service_repository.find_budgets_approved_by_service_information_id(service_information_id.clone()).await?;
        let mut final_cost = Decimal::new(0, 2);
        
        for sb in services_budgets {
            final_cost = final_cost + s.payment_budget_service.calculate_total_cost(sb.travel_cost, sb.service_cost);
        }
        
        let service_order = ServiceOrder::new(
            String::from(""), 
            service_information_id, 
            CurrencyHelper::to_cents(final_cost), 
            dto.description, 
            1, 
            dto.schedule_to, 
            0
        );
        
        s.service_repository.create_service_order(service_order).await?;
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id.clone(),
            title: "Ordem de Serviço criada".to_string(),
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        service_information.service_step_id = 11;
        
        s.service_repository.update_service_information(service_information).await?;
        
        s.notification_service.send(notification, String::from("SERVICE_ORDER_RECEIVED")).await?;
        
        Ok(())
    }
}