use async_trait::async_trait;
use http::StatusCode;
use rust_decimal::Decimal;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::{dto::{create_visit_payment::CreateVisitPaymentDto, payment_response::PaymentResponse}, usecase::usecase::UseCase}, appstate::PaymentAppState, domain::entity::payment_scheduled::PaymentServiceScheduled}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct CreateVisitPaymentUseCase;

type Input = (Claims, CreateVisitPaymentDto);
type Output = Result<PaymentResponse, AppError>;

#[async_trait]
impl UseCase<Input, Output> for CreateVisitPaymentUseCase {
    async fn execute(&self, (user, input): Input, s: PaymentAppState) -> Output {
    
        let optional_service_information = s.service_repository.find_service_information_by_id(input.service_information_id).await?;
    
        let Some(service_information) = optional_service_information else {
            return Err(AppError::new(StatusCode::FORBIDDEN, "Service Information not found for your account"));
        };
    
        let optional_service = s.service_repository.find_service_by_id(service_information.service_id).await?;
        
        let Some(service) = optional_service else {
           	return Err(AppError::new(StatusCode::NOT_FOUND, "Service not found"));
        };
        
        let payment_name = format!("Agendar Serviço: {}", service.name);
    
        let payment_response = s
            .payment_service.create_payment(payment_name, input.method, Decimal::new(service.travel_cost, 2))
            .await?;
        
        // For while in this POC...
        let payment_service_scheduled = PaymentServiceScheduled {
            id: String::from(""),
            schedule_service_information_id: service_information.id,
            user_id: service_information.user_id,
            provider: String::from("ASAAS"), 
            provider_payment_id: payment_response.payment_id.clone(),
            status: String::from("PENDENT"),
            cost: service.travel_cost,
            created_at: 0
        };
        
        s.payment_repository.create_payment_service_scheduled(payment_service_scheduled).await?;
        
        // let notification = Notification {
        //     id: String::from(""),
        //     user_id: user.sub,
        //     title: "".to_string(),
        //     body: "".to_string(),
        //     read: false,
        //     created_at: 0
        // };
        
        // s.notification_service.send(notification, String::from("PAYMENT_VISIT_CREATED")).await?;
        
        Ok(payment_response)
    }
}