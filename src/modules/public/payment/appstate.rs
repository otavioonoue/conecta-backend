use std::sync::Arc;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::{dto::{create_visit_payment::CreateVisitPaymentDto, payment_response::PaymentResponse}, service::payment_service::PaymentService, usecase::usecase::UseCase}, domain::repository::{payment_repository::PaymentRepository, service_repository::ServiceRepository}}}, shared::infra::{error::AppError, service::notification_service::NotificationService}};

#[derive(Clone)]
pub struct PaymentAppState {
    pub payment_service: Arc<dyn PaymentService>,
    pub payment_repository: Arc<dyn PaymentRepository>,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub notification_service: Arc<dyn NotificationService>,
    
    pub create_visit_payment: Arc<dyn UseCase<(Claims, CreateVisitPaymentDto), Result<PaymentResponse, AppError>>> 
}