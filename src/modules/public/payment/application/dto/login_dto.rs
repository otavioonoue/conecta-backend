use serde::Deserialize;
use validator::Validate;

use crate::shared::infra::validate::validators::{email_validation, password_validation};

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(custom(function = email_validation))]
    pub email: String,
    
    #[validate(custom(function = password_validation))]
    pub password: String
}