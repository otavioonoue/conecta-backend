use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{modules::public::consultant::{ConsultantAppState, application::{dto::create_consultant_dto::CreateConsultantDto}}, shared::{infra::error::AppError, presentation::response::DefaultResponse}};

pub fn consultant_router(app_state: ConsultantAppState) -> Router {
  Router::new()
    .route("/", post(create))
    .route("/", get(get_all))
    .with_state(app_state)
}

async fn get_all(
    State(s): State<ConsultantAppState>
) -> Result<impl IntoResponse, AppError> {
  let resp = s.get_all_consultant.execute((), s.clone()).await?;
  
  Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
  State(s): State<ConsultantAppState>,
  Json(dto): Json<CreateConsultantDto>
) -> Result<impl IntoResponse, AppError> {
  let resp = s.create_consultant.execute(dto, s.clone()).await?;
  
  Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}