use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateServiceOrderStatusDto {
    pub status: Status
}

#[derive(Deserialize)]
pub enum Status {
    APPROVED,
    REJECTED
}