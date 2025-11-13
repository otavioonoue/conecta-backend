use std::time::SystemTime;

use async_trait::async_trait;
use jsonwebtoken::{Header, TokenData, encode};
use reqwest::StatusCode;

use crate::{modules::public::auth::{AuthAppState, application::{dto::login_dto::LoginDto, usecase::UseCase}, infrastructure::{constants::JWT_TOKEN, jwt::claim::Claims}, presentation::dto::login_response::LoginResponse}, shared::infra::error::AppError};

pub struct LoginUseCase;

#[async_trait]
impl UseCase<LoginDto, Result<LoginResponse, AppError>> for LoginUseCase {
    async fn execute(&self, dto: LoginDto, s: AuthAppState) -> Result<LoginResponse, AppError> {
        let optional_user = s.auth_repository.find_by_email(dto.email.clone()).await?;
        
        let Some(user) = optional_user else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "This account doesn't exist"));
        };
        
        if dto.email != user.email || dto.password != user.password {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid credentials"))
        }
        
        let exp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 86400;
        
        let claims = Claims {
            sub: user.id,
            email: user.email,
            exp
        };
        
        let token = encode(&Header::default(), &claims, &JWT_TOKEN.encoding).map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
  
        Ok(LoginResponse {
            token
        })
    }
}