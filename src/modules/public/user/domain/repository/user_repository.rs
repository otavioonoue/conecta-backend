use crate::{modules::public::user::domain::entity::{address::Address, user::User}, shared::infra::error::AppError};

pub trait UserRepository {
  async fn create(&self, user: User, address: Address) -> Result<(), AppError>;
  async fn find_all(&self) -> Result<Vec<User>, AppError>;
}