use async_trait::async_trait;

use crate::{modules::public::consultant::{ConsultantAppState, application::usecase::UseCase, domain::entity::consultant::Consultant}, shared::infra::error::AppError};

pub struct FindAllByServiceUseCase;

#[async_trait]
impl UseCase<String, Result<Vec<Consultant>, AppError>> for FindAllByServiceUseCase {
    async fn execute(&self, service_id: String, s: ConsultantAppState) -> Result<Vec<Consultant>, AppError> {
        let consultants = s.consultant_repository.find_all_by_service(service_id).await?;
        Ok(consultants)
    }
}