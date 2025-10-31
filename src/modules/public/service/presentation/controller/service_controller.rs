use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use validator::Validate;

use crate::{modules::public::service::{ServiceAppState, application::dto::create_service_dto::CreateServiceDto}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn service_router(app_state: ServiceAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .with_state(app_state)
}

async fn get_all(
    State(s): State<ServiceAppState>
) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_services.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
    State(s): State<ServiceAppState>,
    Json(dto): Json<CreateServiceDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.create_service.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}