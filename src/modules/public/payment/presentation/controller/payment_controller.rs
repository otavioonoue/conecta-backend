use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::dto::{create_visit_payment::CreateVisitPaymentDto}, appstate::PaymentAppState}}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn payment_router(app_state: PaymentAppState) -> Router {
    Router::new()
        .route("/visit-payment", post(create_visit_payment))
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