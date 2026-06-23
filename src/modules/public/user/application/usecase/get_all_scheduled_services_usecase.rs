use async_trait::async_trait;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{UserAppState, application::usecase::UseCase, domain::entity::scheduled_service_row::ScheduledServiceRow}}, shared::infra::error::AppError};

pub struct GetAllScheduledServicesUseCase;

type Input = Claims;
type Output = Result<Vec<ScheduledServiceRow>, AppError>;

#[async_trait]
impl UseCase<Input, Output> for GetAllScheduledServicesUseCase {
    async fn execute(&self, claims: Input, s: UserAppState) -> Output {
        let mut scheduled_services = s.service_repository.find_all_scheduled_service(claims.sub).await?;
        scheduled_services.iter_mut().for_each(|ss| ss.travel_cost = ss.travel_cost - 10);
        Ok(scheduled_services)
    }
}