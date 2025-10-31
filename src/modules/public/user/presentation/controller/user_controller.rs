use axum::{
  Json, Router,
  extract::State,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};
use validator::Validate;

use crate::{
  modules::public::user::{
    UserAppState, application::dto::create_user_dto::CreateUserDto,
  },
  shared::{presentation::{response::DefaultResponse, types::ApiResult}},
};

pub fn user_router(app_state: UserAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .with_state(app_state)
}

async fn get_all(State(s): State<UserAppState>) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_users.execute((), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create(
    State(s): State<UserAppState>,
    Json(dto): Json<CreateUserDto>,
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    
    let resp = s.create_user.execute(dto, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::CREATED, resp).into_response())
}
