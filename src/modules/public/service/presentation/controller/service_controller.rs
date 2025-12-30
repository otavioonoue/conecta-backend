use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use validator::Validate;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, service::{application::dto::{create_service_dto::CreateServiceDto, schedule_service_dto::ScheduleServiceDto}, appstate::ServiceAppState}}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn service_router(app_state: ServiceAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .route("/schedule_service", post(schedule))
        .with_state(app_state)
}

async fn get_all(
    State(s): State<ServiceAppState>
) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_services.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn get_by_user(
    State(s): State<ServiceAppState>
) -> ApiResult<impl IntoResponse> {
    let resp = ();
    
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

async fn schedule(
    claim: Claims,
    State(s): State<ServiceAppState>,
    Json(dto): Json<ScheduleServiceDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.schedule_service.execute((claim, dto), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}