use std::{sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{modules::public::user::{application::dto::create_user_dto::CreateUserDto, domain::usecase::UseCase, UserAppState}, shared::{infra::error::AppError, presentation::response::DefaultResponse}};

pub fn user_router(app_state: Arc<UserAppState>) -> Router {
  Router::new()
    .route("/", post(create))
    .route("/", get(get_all))
    .with_state(app_state)
}

async fn get_all(
    State(s): State<Arc<UserAppState>>
) -> Result<impl IntoResponse, AppError> {
    let resp = s.get_all_users.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
  State(s): State<Arc<UserAppState>>,
  Json(dto): Json<CreateUserDto>
) -> Result<impl IntoResponse, AppError> {
    let resp = s.create_user.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}