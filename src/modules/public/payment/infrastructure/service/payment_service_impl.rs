use std::env;
use async_trait::async_trait;
use http::StatusCode;
use reqwest::{Client};
use rust_decimal::Decimal;

use crate::{modules::public::payment::{application::{dto::payment_response::PaymentResponse, service::payment_service::PaymentService, types::payment_method_type::PaymentMethodType}, infrastructure::dto::create_payment::{CreatePaymentRequest, CreatePaymentResponse}}, shared::infra::error::AppError};

pub struct PaymentServiceBase<T> {
	client: T,
	base_url: String,
	access_token: String
}

impl PaymentServiceBase<Client> {
	pub fn new() -> Self {
		let base_url = env::var("API_PAYMENT_URL").expect("Couldn't find API_PAYMENT_URL");
		let access_token = format!("${}", env::var("API_PAYMENT_TOKEN").expect("Couldn't find API_PAYMENT_TOKEN"));
		
		PaymentServiceBase {
			client: Client::new(),
			base_url,
			access_token: access_token
		}
	}
}

pub struct PaymentServiceAsaasImpl<T> {
	payment_service: PaymentServiceBase<T>
}

impl PaymentServiceAsaasImpl<Client> {
	pub fn new() -> Self {
		PaymentServiceAsaasImpl {
			payment_service: PaymentServiceBase::new()
		}
	}
}

#[async_trait]
impl PaymentService for PaymentServiceAsaasImpl<Client> {
	async fn create_payment(&self, payment_name: String, method: PaymentMethodType, value: Decimal) -> Result<PaymentResponse, AppError> {
		let url = format!("{}/paymentLinks", &self.payment_service.base_url);
		
		let method_string = match method {
			PaymentMethodType::Pix => "PIX",
			PaymentMethodType::Credit => "CREDIT_CARD",
			PaymentMethodType::Debit => "CREDIT_CARD",
		};
		
		let body = CreatePaymentRequest {
			payment_name,
			billing_type: method_string.to_string(),
			value,
			charge_type: String::from("DETACHED"),
			due_date_limit_days: 1
		};
		
		let resp: CreatePaymentResponse = self.payment_service.client.post(url)
			.json(&body)
			.header("access_token", &self.payment_service.access_token)
			.header(reqwest::header::USER_AGENT, "ConectaBrokersApp/1.0")
			.send()
			.await
			.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Couldn't send payment."))?
			.error_for_status()
			.map_err(|e| {
				println!("Error: {}", e);
				AppError::new(
				StatusCode::BAD_GATEWAY,
				"Payment provider returned an error."
			)})?
			.json()
			.await
			.expect("Couldn't transform payment response in json");
		
		let payment = PaymentResponse { 
			payment_id: resp.id,
			billing_type: method, 
			url: resp.url
		};
		
		Ok(payment)
	}
}