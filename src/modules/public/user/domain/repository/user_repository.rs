use async_trait::async_trait;

use crate::{modules::public::user::domain::entity::{address::Address, user::User}, shared::infra::error::AppError};

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn create(&self, user: User, address: Address) -> Result<(), AppError>;
  async fn find_all(&self) -> Result<Vec<User>, AppError>;
}