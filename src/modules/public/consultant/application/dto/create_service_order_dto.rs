use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateServiceOrderDto {
    pub description: String,
    pub schedule_to: i64
}