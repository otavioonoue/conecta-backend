use core::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, Visitor}};
use serde_json::Value;
use validator::ValidationErrors;

use crate::shared::infra::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse<T> {
    #[serde(serialize_with = "status_code_as_u16", deserialize_with = "u16_as_status_code")]
    pub code: StatusCode,
    pub success: bool,
    pub data: T
}

pub fn status_code_as_u16<S: Serializer>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(code.as_u16())
}

pub fn u16_as_status_code<'de, D: Deserializer<'de>>(deserializer: D) -> Result<StatusCode, D::Error> {
    struct StatusCodeVisitor;
    
    impl<'de> Visitor<'de> for StatusCodeVisitor {
        type Value = StatusCode;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid u16 representing a StatusCode")
        }

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            StatusCode::from_u16(value).map_err(|_| E::custom(format!("invalid StatusCode: {}", value)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value < 0 || value > u16::MAX as i64 {
                return Err(E::custom(format!("invalid StatusCode: {}", value)));
            }
            StatusCode::from_u16(value as u16).map_err(|_| E::custom(format!("invalid StatusCode: {}", value)))
        }
    }

    deserializer.deserialize_u16(StatusCodeVisitor)
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

impl From<ValidationErrors> for DefaultResponse<Value> {
    fn from(err: validator::ValidationErrors) -> Self {
        let json = serde_json::to_value(&err).unwrap_or_else(|_| serde_json::json!({}));   
        DefaultResponse::error(StatusCode::BAD_REQUEST, json)
    }
}

impl From<AppError> for DefaultResponse<Value>{
    fn from(err: AppError) -> Self {
        let json = serde_json::to_value(&err.message).unwrap_or_else(|_| serde_json::json!({}));
        DefaultResponse::error(err.code, json)
    }
}
