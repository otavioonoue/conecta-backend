use async_trait::async_trait;

use crate::modules::public::user::UserAppState;

pub mod create_user_usecase;
pub mod get_all_users_usecase;
pub mod get_all_addresses_usecase;
pub mod create_address_usecase;

#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
  async fn execute(&self, input: I, s: UserAppState) -> O;
}