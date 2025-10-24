use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{modules::public::user::{application::dto::create_user_dto::CreateUserDto, domain::usecase::UseCase, UserAppState}, shared::presentation::response::DefaultResponse};

pub fn user_router(app_state: Arc<UserAppState>) -> Router {
  Router::new()
    .route("/", post(create))
    .route("/", get(get_all))
    .with_state(app_state)
}

async fn get_all(
    State(s): State<Arc<UserAppState>>
) -> impl IntoResponse {
    let resp = s.get_all_users.execute((), s.clone()).await;
    
    if let Err(e) = resp {
        return serde_json::to_string(&DefaultResponse::new(StatusCode::INTERNAL_SERVER_ERROR, false, e.to_string())).unwrap()
    }
    
    serde_json::to_string(&DefaultResponse::new(StatusCode::OK, true, resp.unwrap())).unwrap()
}

async fn create(
  State(s): State<Arc<UserAppState>>,
  Json(dto): Json<CreateUserDto>
) -> impl IntoResponse {
    let resp = s.create_user.execute(dto, s.clone()).await;
    
    if let Err(e) = resp {
        return serde_json::to_string(&DefaultResponse::new(StatusCode::INTERNAL_SERVER_ERROR, false, e.to_string())).unwrap();
    }
    
    serde_json::to_string(&DefaultResponse::new(StatusCode::CREATED, true, "Created!")).unwrap()
}