use async_trait::async_trait;
use http::StatusCode;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, user::{UserAppState, application::{dto::{update_service_order_status_dto::{Status, UpdateServiceOrderStatusDto}}, usecase::UseCase}}}, shared::{domain::entity::notification::Notification, infra::error::AppError}};

pub struct UpdateServiceOrderStatusUseCase;

type Input = (Claims, String, UpdateServiceOrderStatusDto);
type Output = Result<(), AppError>;

#[async_trait]
impl UseCase<Input, Output> for UpdateServiceOrderStatusUseCase {
    async fn execute(&self, (claims, service_order_id, service_order_dto): Input, s: UserAppState) -> Output {
        let Some(mut service_order) = s.service_repository.find_service_order_by_id(service_order_id.clone()).await? else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service order not found"));
        };
        
        let service_information_optional = s.service_repository.find_service_information_by_service_order_id(service_order_id).await?;
        
        let Some(mut service_information) = service_information_optional else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        };
        
        if claims.sub != service_information.user_id {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Service Information not found"));
        }
        
        if service_information.service_step_id != 11 {
            return Err(AppError::new(StatusCode::UNPROCESSABLE_ENTITY, "The service order hasn't been created yet"))
        }
        
        if let Status::APPROVED = service_order_dto.status {
            service_order.service_order_status_id = 2;
            service_information.service_step_id = 12;
        } else {
            service_order.service_order_status_id = 3;
            service_information.service_step_id = 13;
        }
        
        s.service_repository.update_service_order_status(service_order).await?;
        
        s.service_repository.update_service_information(service_information.clone()).await?;
        
        let mut title_notification = String::from("");
        let mut status_notification = String::from("");
    
        match service_order_dto.status {
            Status::APPROVED => {
                title_notification = String::from("Ordem de Serviço aceita");
                status_notification = String::from("SERVICE_ORDER_CONFIRMED");
            },
            Status::REJECTED => {
                title_notification = String::from("Ordem de Serviço recusada");
                status_notification = String::from("SERVICE_ORDER_DENIED");
            }
        }
        
        let notification = Notification {
            id: String::from(""),
            user_id: service_information.user_id,
            title: title_notification,
            body: "".to_string(),
            read: false,
            created_at: 0
        };
        
        s.notification_service.send(notification, status_notification).await?;
        Ok(())
    }
}