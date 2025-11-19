use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}};
use validator::Validate;

use crate::{modules::public::auth::{AuthAppState, application::dto::login_dto::LoginDto, infrastructure::jwt::claim::Claims}, shared::presentation::{response::DefaultResponse, types::ApiResult}};

pub fn auth_router(app_state: AuthAppState) -> Router {
    Router::new()
        .route("/user", post(login))
        .route("/consultant", post(login_consultant))
        .route("/a", get(public))
        .route("/", get(private))
        .with_state(app_state)
}

async fn login(
    State(s): State<AuthAppState>,
    Json(dto): Json<LoginDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.login.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn login_consultant(
    State(s): State<AuthAppState>,
    Json(dto): Json<LoginDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    let resp = s.login_consultant.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}

async fn public() -> ApiResult<impl IntoResponse> {
    Ok("public route")
}

async fn private(claims: Claims) -> ApiResult<impl IntoResponse> {
    Ok(format!("Hello, {:#?}", claims))
}