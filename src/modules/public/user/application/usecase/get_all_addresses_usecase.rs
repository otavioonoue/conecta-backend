use async_trait::async_trait;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{UserAppState, application::usecase::UseCase, domain::entity::address::Address}}, shared::infra::error::AppError};

pub struct GetAllAddressesUseCase;

#[async_trait]
impl UseCase<Claims, Result<Vec<Address>, AppError>> for GetAllAddressesUseCase {
  async fn execute(&self, user: Claims, s: UserAppState) -> Result<Vec<Address>, AppError> {
    let addresses = s.address_repository.find_all(user.sub).await?;
    
    Ok(addresses)
  }
}