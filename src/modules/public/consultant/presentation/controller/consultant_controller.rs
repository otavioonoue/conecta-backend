use axum::{Json, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post}};
use validator::Validate;

use crate::{modules::public::consultant::{ConsultantAppState, application::dto::{add_service_dto::AddServiceDto, create_consultant_dto::CreateConsultantDto, remove_service_dto::RemoveServiceDto}}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn consultant_router(app_state: ConsultantAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .route("/service/{consultant_id}", post(add_service))
        .route("/service/{consultant_id}", delete(remove_service))
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

async fn add_service(
    State(s): State<ConsultantAppState>,
    Path(consultant_id): Path<String>,
    Json(dto): Json<AddServiceDto>,
) -> ApiResult<impl IntoResponse> {
    let resp = s.add_service.execute((dto, consultant_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn remove_service(
    State(s): State<ConsultantAppState>,
    Path(consultant_id): Path<String>,
    Json(dto): Json<RemoveServiceDto>
) -> ApiResult<impl IntoResponse> {
    let resp = s.remove_service.execute((dto, consultant_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}