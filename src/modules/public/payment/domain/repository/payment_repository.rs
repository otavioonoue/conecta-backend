use async_trait::async_trait;

use crate::{modules::public::auth::domain::entity::{consultant::Consultant}, shared::infra::error::AppError};

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    // async fn find_by_email_consultant(&self, consultant_email: String) -> Result<Option<Consultant>, AppError>;
}