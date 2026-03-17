use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, consultant::{application::usecase::usecase::UseCase, appstate::ConsultantAppState}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct FinishOrderServiceUseCase;

type Input = (Claims, String);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for FinishOrderServiceUseCase {
    async fn execute(&self, (claims, service_information_id): Input, s: ConsultantAppState) -> Output {
        let service_information_optional = s.service_repository.find_service_information_by_id(service_information_id).await?;
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        if service_information.service_step_id != 12 {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "The service order hasn't been confirmed yet"))
        }
        
        service_information.consultant_id = Some(claims.sub);
        service_information.service_step_id = 14;
        
        let optional_service_order = s.service_repository.find_service_order_by_service_information_id(service_information.id.clone()).await?;
        
        let Some(mut service_order) = optional_service_order else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service order not found"))
        };
        
        service_order.service_order_status_id = 4;
        
        s.consultant_repository.update_service_information_status(service_information.clone()).await?;
        
        s.service_repository.update_service_order_status(service_order).await?;
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id,
            title: "Serviço finalizado".to_string(),
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.notification_service.send(notification, String::from("SERVICE_ORDER_FINISHED")).await?;
        
        Ok(())
    }
}