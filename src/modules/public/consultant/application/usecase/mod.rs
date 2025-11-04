use async_trait::async_trait;

use crate::modules::public::consultant::ConsultantAppState;

pub mod create_consultant_usecase;
pub mod get_all_consultant_usecase;
pub mod add_service_consultant_usecase;
pub mod remove_service_consultant_usecase;


#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
  async fn execute(&self, input: I, s: ConsultantAppState) -> O;
}