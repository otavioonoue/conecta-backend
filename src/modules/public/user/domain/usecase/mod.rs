use std::sync::Arc;

use crate::modules::public::user::UserAppState;

pub trait UseCase<I, O> {
  async fn execute(&self, input: I, s: Arc<UserAppState>) -> O;
}