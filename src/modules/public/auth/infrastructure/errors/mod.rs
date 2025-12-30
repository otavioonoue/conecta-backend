use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InvalidAccess
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::InvalidAccess => (StatusCode::UNAUTHORIZED, "This account is not active")
        };
        
        let body = Json(json!({
            "error": error_message
        }));
        
        (status, body).into_response()
    }
}