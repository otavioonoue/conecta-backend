use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::{modules::public::payment::application::{dto::payment_response::PaymentResponse, types::payment_method_type::PaymentMethodType}, shared::infra::error::AppError};

#[async_trait]
pub trait PaymentService: Send + Sync {
    async fn create_payment(&self, payment_name: String, method: PaymentMethodType, value: Decimal) -> Result<PaymentResponse, AppError>;
}