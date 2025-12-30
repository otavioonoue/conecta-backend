use async_trait::async_trait;

use crate::{modules::public::payment::domain::entity::service::Service, shared::infra::error::AppError};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn find_by_id(&self, service_id: String) -> Result<Option<Service>, AppError>;
}