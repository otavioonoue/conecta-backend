use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::payment::{application::{dto::webhook_event_request::WebhookEventRequest, usecase::usecase::UseCase}, appstate::PaymentAppState, domain::entity::service_payment::PaymentKind}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct WebHookPaymentNotificationUseCase;

type Input = WebhookEventRequest;
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for WebHookPaymentNotificationUseCase {
    async fn execute(&self, wh_event: Input, s: PaymentAppState) -> Output {
        let Some(payment_link) = wh_event.payment.paymentLink else {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Doesn't have payment link"));
        };
        
        let service_payment_optional = s.payment_repository.find_by_payment_link(payment_link).await?;
        
        let Some(mut service_payment) = service_payment_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Payment not found"));
        };
        
        if wh_event.event == String::from("PAYMENT_CREATED") {
            service_payment.status = String::from("PENDING");
            
            s.payment_repository.update_payment(service_payment.clone()).await?;
            
            let service_information_optional = s.service_repository.find_service_information_by_id(service_payment.schedule_service_information_id)
                .await?;
            
            let Some(mut service_information) = service_information_optional else {
                return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, ""));
            };
            
            match service_payment.kind {
                PaymentKind::Budget => service_information.service_step_id = 9,
                PaymentKind::Scheduled => service_information.service_step_id = 3,
            }
            
            s.service_repository.update_service_information(service_information.clone()).await?;
            
            let notification = Notification {
                id: String::from(""),
                user_id: service_information.user_id,
                title: "".to_string(),
                body: "".to_string(),
                read: false,
                created_at: 0
            };
            
            s.notification_service.send(notification, String::from("PAYMENT_CREATED")).await?;
        }
        else if wh_event.event == String::from("PAYMENT_RECEIVED") {
            service_payment.status = String::from("COMPLETED");
            
            s.payment_repository.update_payment(service_payment.clone()).await?;
            
            let service_information_optional = s.service_repository.find_service_information_by_id(service_payment.schedule_service_information_id)
                .await?;
            
            let Some(mut service_information) = service_information_optional else {
                return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, ""));
            };
            
            match service_payment.kind {
                PaymentKind::Budget => service_information.service_step_id = 10,
                PaymentKind::Scheduled => service_information.service_step_id = 4,
            }
            
            s.service_repository.update_service_information(service_information.clone()).await?;
            
            let notification = Notification {
                id: String::from(""),
                user_id: service_information.user_id,
                title: "".to_string(),
                body: "".to_string(),
                read: false,
                created_at: 0
            };
            
            s.notification_service.send(notification, String::from("PAYMENT_RECEIVED")).await?;
        }
        
        Ok(())
    }
}