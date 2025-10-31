use axum::
    http::StatusCode
;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct AppError {
    #[serde(serialize_with = "status_code_as_u16")]
    pub code: StatusCode,
    pub message: String
}

pub fn status_code_as_u16<S: Serializer>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(code.as_u16())
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}