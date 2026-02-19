use std::time::SystemTime;

use async_trait::async_trait;
use http::StatusCode;
use jsonwebtoken::{Header, encode};

use crate::{modules::public::auth::{application::{dto::login_dto::LoginDto, usecase::UseCase}, appstate::AuthAppState, infrastructure::{constants::JWT_TOKEN, jwt::{claim::Claims, role::Role}}, presentation::dto::login_response::LoginResponse}, shared::infra::error::AppError};

pub struct LoginAdminUseCase;

#[async_trait]
impl UseCase<LoginDto, Result<LoginResponse, AppError>> for LoginAdminUseCase {
    async fn execute(&self, dto: LoginDto, s: AuthAppState) -> Result<LoginResponse, AppError> {
        let optional_consultant = s.auth_repository.find_by_email_admin(dto.email.clone()).await?;
        
        let Some(consultant) = optional_consultant else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "This account doesn't exist"));
        };
        
        if dto.email != consultant.email || !s.hash_service.compare(&dto.password, &consultant.password) {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid credentials"))
        }
        
        if !consultant.active {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "This account is not active"))
        }
        
        let exp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 86400;
        
        let claims = Claims {
            sub: consultant.id,
            name: consultant.name,
            email: consultant.email,
            active: consultant.active,
            role: Role::ADMIN,
            exp,
        };
        
        let token = encode(&Header::default(), &claims, &JWT_TOKEN.encoding).map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
  
        Ok(LoginResponse {
            token
        })
    }
}