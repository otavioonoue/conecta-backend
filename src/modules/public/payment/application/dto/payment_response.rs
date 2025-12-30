use serde::{Deserialize, Serialize};

use crate::modules::public::payment::application::types::payment_method_type::PaymentMethodType;

#[derive(Serialize, Deserialize)]
pub struct PaymentResponse {
	pub payment_id: String,
	pub billing_type: PaymentMethodType,
	pub url: String
}