use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct BudgetStatusDto {
    pub status: Status
}

#[derive(Deserialize)]
pub enum Status {
    APPROVED,
    REJECTED
}