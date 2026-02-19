use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::payment::{application::{dto::webhook_event_request::WebhookEventRequest, usecase::usecase::UseCase}, appstate::PaymentAppState}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct WebHookPaymentNotificationUseCase;

type Input = WebhookEventRequest;
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for WebHookPaymentNotificationUseCase {
    async fn execute(&self, wh_event: Input, s: PaymentAppState) -> Output {
        let Some(payment_link) = wh_event.payment.paymentLink else {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Doesn't have payment link"));
        };
        
        let payment_service_scheduled_optional = s.payment_repository.find_by_payment_link(payment_link).await?;
        
        let Some(mut payment_service_scheduled) = payment_service_scheduled_optional else {
            return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, ""));
        };
        
        if wh_event.event == String::from("PAYMENT_RECEIVED") {
            payment_service_scheduled.status = String::from("COMPLETED");
            
            s.payment_repository.update_payment(payment_service_scheduled.clone()).await?;
            
            let service_information_optional = s.service_repository.find_service_information_by_id(payment_service_scheduled.schedule_service_information_id)
                .await?;
            
            let Some(mut service_information) = service_information_optional else {
                return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, ""));
            };
            
            service_information.service_step_id = 3;
            
            s.service_repository.update_service_information(service_information.clone()).await?;
            
            let notification = Notification {
                id: String::from(""),
                user_id: service_information.user_id,
                title: "".to_string(),
                body: "".to_string(),
                read: false,
                created_at: 0
            };
            
            s.notification_service.send(notification, String::from("PAYMENT_VISIT_RECEIVED")).await?;
        }
        
        Ok(())
    }
}