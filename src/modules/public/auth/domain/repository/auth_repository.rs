use async_trait::async_trait;

use crate::{modules::public::auth::domain::entity::{admin::Admin, consultant::Consultant, user::User}, shared::infra::error::AppError};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_email_admin(&self, admin_email: String) -> Result<Option<Admin>, AppError>;
    async fn find_by_email_consultant(&self, consultant_email: String) -> Result<Option<Consultant>, AppError>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError>;
}