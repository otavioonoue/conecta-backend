use axum::{Json, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post}};
use validator::Validate;

use crate::{modules::public::{auth::infrastructure::jwt::claim::{AdminOnly, Claims}, consultant::{application::dto::{add_service_dto::AddServiceDto, create_budget_dto::CreateBudgetDto, create_consultant_dto::CreateConsultantDto, create_service_order_dto::CreateServiceOrderDto, remove_service_dto::RemoveServiceDto}, appstate::ConsultantAppState}}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn consultant_router(app_state: ConsultantAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .route("/service/{consultant_id}", post(add_service))
        .route("/service/{consultant_id}", delete(remove_service))
        .route("/find_all_by_service/{service_id}", get(find_all_by_service))
        .route("/service/confirm_service_scheduled/{service_information_id}", post(confirm_service_scheduled))
        .route("/service/budget/{service_information_id}", post(create_budget))
        .route("/service/service_order/{service_information_id}", post(create_service_order))
        .route("/service/finish/{service_information_id}", post(finish_order_service))
        .with_state(app_state)
}

async fn get_all(
    State(s): State<ConsultantAppState>,
    AdminOnly(_claims): AdminOnly
) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_consultant.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
    State(s): State<ConsultantAppState>,
    AdminOnly(_claims): AdminOnly,
    Json(dto): Json<CreateConsultantDto>,
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.create_consultant.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn add_service(
    State(s): State<ConsultantAppState>,
    AdminOnly(_claims): AdminOnly,
    Path(consultant_id): Path<String>,
    Json(dto): Json<AddServiceDto>,
) -> ApiResult<impl IntoResponse> {
    let resp = s.add_service.execute((dto, consultant_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn remove_service(
    State(s): State<ConsultantAppState>,
    AdminOnly(_claims): AdminOnly,
    Path(consultant_id): Path<String>,
    Json(dto): Json<RemoveServiceDto>
) -> ApiResult<impl IntoResponse> {
    let resp = s.remove_service.execute((dto, consultant_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn find_all_by_service(
    State(s): State<ConsultantAppState>,
    AdminOnly(_claims): AdminOnly,
    Path(service_id): Path<String>
) -> ApiResult<impl IntoResponse> {
    let resp = s.find_all_by_service.execute(service_id, s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn confirm_service_scheduled(
    State(s): State<ConsultantAppState>,
    claims: Claims,
    Path(service_information_id): Path<String>
) -> ApiResult<impl IntoResponse> {
    let resp = s.confirm_scheduled_service.execute((claims, service_information_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn create_budget(
    State(s): State<ConsultantAppState>,
    claims: Claims,
    Path(service_information_id): Path<String>,
    Json(dto): Json<CreateBudgetDto>
) -> ApiResult<impl IntoResponse> {
    let resp = s.create_service_budget.execute((claims, service_information_id, dto), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn create_service_order(
    State(s): State<ConsultantAppState>,
    claims: Claims,
    Path(service_information_id): Path<String>,
    Json(dto): Json<CreateServiceOrderDto>
) -> ApiResult<impl IntoResponse> {
    let resp = s.create_service_order.execute((claims, service_information_id, dto), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn finish_order_service(
    State(s): State<ConsultantAppState>,
    claims: Claims,
    Path(service_information_id): Path<String>
) -> ApiResult<impl IntoResponse> {
    let resp = s.finish_service_order.execute((claims, service_information_id), s.clone()).await?;
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}