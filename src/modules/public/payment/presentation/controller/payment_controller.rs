use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::dto::{create_budget_payment::CreateBudgetPaymentDto, create_visit_payment::CreateVisitPaymentDto, webhook_event_request::WebhookEventRequest}, appstate::PaymentAppState}}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn payment_router(app_state: PaymentAppState) -> Router {
    Router::new()
        .route("/webhook-payment-notification", post(webhook_payment_notification))
        .route("/visit-payment", post(create_visit_payment))
        .route("/budget-payment", post(create_budget_payment))
        .with_state(app_state)
}

async fn create_visit_payment(
    State(s): State<PaymentAppState>,
    claims: Claims,
    Json(dto): Json<CreateVisitPaymentDto>,
) -> ApiResult<impl IntoResponse> {
    let resp = s.create_visit_payment.execute((claims, dto), s.clone()).await?;
	
	Ok(DefaultResponse::new(StatusCode::CREATED, true, resp).into_response())
}

async fn create_budget_payment(
    State(s): State<PaymentAppState>,
    claims: Claims,
    Json(dto): Json<CreateBudgetPaymentDto>,
) -> ApiResult<impl IntoResponse> {
    let resp = s.create_budget_payment.execute((claims, dto), s.clone()).await?;
	
	Ok(DefaultResponse::new(StatusCode::CREATED, true, resp).into_response())
}

async fn webhook_payment_notification(
    State(s): State<PaymentAppState>,
    Json(dto): Json<WebhookEventRequest>
) -> ApiResult<impl IntoResponse> {
    let resp = s.wh_payment_notification.execute(dto, s.clone()).await?;
    Ok(DefaultResponse::new(StatusCode::OK, true, resp).into_response())
}