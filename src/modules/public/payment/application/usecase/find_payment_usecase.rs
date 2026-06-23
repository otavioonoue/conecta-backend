use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::{usecase::usecase::UseCase}, appstate::PaymentAppState, domain::entity::service_payment::ServicePayment}}, shared::infra::error::AppError};

pub struct FindPaymentUseCase;

type Input = (Claims, String, String);
type Output = Result<ServicePayment, AppError>;

#[async_trait]
impl UseCase<Input, Output> for FindPaymentUseCase {
    async fn execute(&self, (claims, service_information_id, kind): Input, s: PaymentAppState) -> Output {
        let optional_service_payment = s.payment_repository.find_by_service_information_id(service_information_id, kind).await?;
        
        let Some(service_payment) = optional_service_payment else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Payment not found."))
        };
        
        if service_payment.user_id != claims.sub {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Payment not found."))
        }
        
        Ok(service_payment)
    }
}