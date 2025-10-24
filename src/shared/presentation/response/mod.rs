use axum::http::StatusCode;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct DefaultResponse<T> {
    #[serde(serialize_with = "status_code_as_u16")]
    code: StatusCode,
    success: bool,
    data: T
}

fn status_code_as_u16<S: Serializer>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(code.as_u16())
}

impl<T> DefaultResponse<T> {
    pub fn new(code: StatusCode, success: bool, data: T) -> Self {
        DefaultResponse { code, success, data }
    }
}