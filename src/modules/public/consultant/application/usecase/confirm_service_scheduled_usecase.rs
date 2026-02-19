use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, consultant::{application::usecase::usecase::UseCase, appstate::ConsultantAppState}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct ConfirmServiceScheduledUseCase;

type Input = (Claims, String);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for ConfirmServiceScheduledUseCase {
    async fn execute(&self, (claims, service_information_id): Input, s: ConsultantAppState) -> Output {
        let service_information_optional = s.service_repository.find_service_information_by_id(service_information_id).await?;
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        if service_information.service_step_id != 3 {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "The payment hasn't been received yet"))
        }
        
        service_information.consultant_id = Some(claims.sub);
        service_information.service_step_id = 4;
        
        s.consultant_repository.confirm_scheduled_service(service_information.consultant_id.clone().unwrap(), service_information.clone()).await?;
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id,
            title: "Visita Confirmada".to_string(),
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.notification_service.send(notification, String::from("VISIT_CONFIRMED")).await?;
        
        Ok(())
    }
}