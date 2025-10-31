use async_trait::async_trait;

use crate::modules::public::service::ServiceAppState;

pub mod create_service_usecase;
pub mod get_all_service_usecase;

#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
  async fn execute(&self, input: I, s: ServiceAppState) -> O;
}