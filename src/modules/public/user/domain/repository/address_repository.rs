use async_trait::async_trait;

use crate::{modules::public::user::domain::entity::{address::Address}, shared::infra::error::AppError};

#[async_trait]
pub trait AddressRepository: Send + Sync {
  async fn create(&self, user_id: String, address: Address) -> Result<(), AppError>;
  async fn find_all(&self, user_id: String) -> Result<Vec<Address>, AppError>;
}