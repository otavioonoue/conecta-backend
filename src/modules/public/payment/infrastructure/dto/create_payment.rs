use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
	#[serde(rename = "name")]
    pub payment_name: String,

    #[serde(rename = "billingType")]
    pub billing_type: String, // PIX, CREDIT_CARD
    
    pub value: Decimal,
    
    #[serde(rename = "chargeType")]
    pub charge_type: String, // DETACHED, RECURRENT, INSTALLMENT

    #[serde(rename = "dueDateLimitDays")]
    pub due_date_limit_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentResponse {
    pub id: String,
    
    #[serde(rename = "billingType")]
    pub billing_type: String,
    
    pub value: Decimal,
    
    pub url: String
}
