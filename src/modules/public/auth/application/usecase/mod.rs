use async_trait::async_trait;

use crate::modules::public::auth::AuthAppState;

pub mod login_usecase;
pub mod login_consultant_usecase;

#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
  async fn execute(&self, input: I, s: AuthAppState) -> O;
}