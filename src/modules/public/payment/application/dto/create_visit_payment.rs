use serde::{Deserialize, Serialize};

use crate::{modules::public::payment::application::types::payment_method_type::PaymentMethodType};

#[derive(Deserialize, Serialize)]
pub struct CreateVisitPaymentDto {
    pub service_information_id: String,
    pub method: PaymentMethodType
}