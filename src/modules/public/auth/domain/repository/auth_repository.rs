use async_trait::async_trait;

use crate::{modules::public::auth::domain::entity::user::User, shared::infra::error::AppError};

#[async_trait]
pub trait AuthRepository: Send + Sync {
  // async fn find_by_id(&self, consultant_id: String) -> Result<Option<User>, AppError>;
  async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError>;
}