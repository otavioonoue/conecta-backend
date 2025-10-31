use crate::shared::presentation::response::DefaultResponse;

pub type ApiResult<T> = Result<T, DefaultResponse<serde_json::Value>>;