use async_trait::async_trait;

use crate::modules::public::consultant::appstate::ConsultantAppState;

#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
  async fn execute(&self, input: I, s: ConsultantAppState) -> O;
}