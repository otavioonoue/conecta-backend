use axum::{
  Json, Router,
  extract::State,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};
use validator::Validate;

use crate::{
  modules::public::{auth::infrastructure::jwt::claim::Claims, user::{
      UserAppState, application::dto::{create_address_dto::CreateAddressDto, create_user_dto::CreateUserDto},
  }},
  shared::presentation::{response::DefaultResponse, types::ApiResult},
};

pub fn user_router(app_state: UserAppState) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .route("/addresses", get(get_all_addresses))
        .route("/address", post(create_address))
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

async fn get_all_addresses(
    claims: Claims,
    State(s): State<UserAppState>
) -> ApiResult<impl IntoResponse> {
    let resp = s.get_all_addresses.execute(claims, s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}

async fn create_address(
    claims: Claims,
    State(s): State<UserAppState>,
    Json(dto): Json<CreateAddressDto>
) -> ApiResult<impl IntoResponse> {
    dto.validate()?;
    
    let resp = s.create_address.execute((claims, dto), s.clone()).await?;
    
    Ok(DefaultResponse::ok(StatusCode::OK, resp).into_response())
}
