use async_trait::async_trait;
use http::StatusCode;
use rust_decimal::Decimal;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::{dto::{create_visit_payment::CreateVisitPaymentDto, payment_response::PaymentResponse}, usecase::usecase::UseCase}, appstate::PaymentAppState}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct CreateVisitPaymentUseCase;

#[async_trait]
impl UseCase<(Claims, CreateVisitPaymentDto), Result<PaymentResponse, AppError>> for CreateVisitPaymentUseCase {
	async fn execute(&self, (user, input): (Claims, CreateVisitPaymentDto), s: PaymentAppState) -> Result<PaymentResponse, AppError> {
        let optional_service = s.service_repository.find_by_id(input.service_id).await?;
        
        let Some(service) = optional_service else {
           	return Err(AppError::new(StatusCode::NOT_FOUND, "Service not found"));
        };
        
        let payment_name = format!("Agendar Serviço: {}", service.name);

        let payment_response = s
            .payment_service.create_payment(payment_name, input.method, Decimal::new(service.travel_cost, 2))
            .await?;
        
        let notification = Notification {
            id: String::from(""),
            user_id: user.sub,
            title: "".to_string(),
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.notification_service.send(notification, String::from("PAYMENT_VISIT_DONE")).await?;
        
        Ok(payment_response)
	}
}