use async_trait::async_trait;

use crate::{modules::public::service::domain::entity::address::Address, shared::infra::error::AppError};

#[async_trait]
pub trait AddressRepository: Send + Sync {
    async fn find_by_id(&self, address_id: String) -> Result<Option<Address>, AppError>;
}