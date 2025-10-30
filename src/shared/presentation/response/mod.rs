use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct DefaultResponse<T> {
    #[serde(serialize_with = "status_code_as_u16")]
    pub code: StatusCode,
    pub success: bool,
    pub data: T
}

pub fn status_code_as_u16<S: Serializer>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(code.as_u16())
}

impl<T> DefaultResponse<T> {
    pub fn new(code: StatusCode, success: bool, data: T) -> Self {
        DefaultResponse { code, success, data }
    }
    
    pub fn ok(code: StatusCode, data: T) -> Self {
        DefaultResponse { 
            code, 
            success: true, 
            data 
        }
    }
    
    pub fn error(code: StatusCode, data: T) -> Self {
        DefaultResponse { 
            code, 
            success: false, 
            data 
        }
    }
}

impl<T: Serialize> IntoResponse for DefaultResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = self.code;
        let body = Json(self);
        (status, body).into_response()
    }
}