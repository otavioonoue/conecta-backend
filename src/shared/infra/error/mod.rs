use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "code": self.code.as_u16(),
            "success": false,
            "error": self.message,
        }));
        (self.code, body).into_response()
    }
}
