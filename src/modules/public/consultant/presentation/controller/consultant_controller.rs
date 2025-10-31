use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use validator::Validate;

use crate::{modules::public::consultant::{ConsultantAppState, application::dto::create_consultant_dto::CreateConsultantDto}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn consultant_router(app_state: ConsultantAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .with_state(app_state)
}

async fn get_all(
    State(s): State<ConsultantAppState>
) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_consultant.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
    State(s): State<ConsultantAppState>,
    Json(dto): Json<CreateConsultantDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.create_consultant.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}