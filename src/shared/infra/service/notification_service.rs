use std::str::FromStr;

use async_trait::async_trait;
use http::StatusCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::shared::{application::ws::ws::Ws, domain::entity::{notification::Notification, service_schedule::ServiceSchedule}, infra::{database::{db_config::{Database, Db}}, error::AppError, ws::ws_impl::WsHub}};

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send(&self, notification: Notification, new_status: String) -> Result<(), AppError>;
    async fn visit_payment_done(&self, service_schedule: ServiceSchedule) -> Result<(), AppError>;
}

pub struct NotificationServiceImpl<T: Db> {
    pub db: T,
    pub ws_notification: WsHub
}

impl<T: Db> NotificationServiceImpl<T> {
    pub fn new(app_state: T, ws_notification: WsHub) -> Self {
        NotificationServiceImpl { db: app_state, ws_notification }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl<Database<Pool<Postgres>>> {
    async fn send(&self, mut notification: Notification, new_status: String) -> Result<(), AppError> {
        let (title, body) = match new_status.as_str() {
            "PAYMENT_VISIT_DONE" => ("Pagamento realizado", ""),
            "VISIT_CONFIRMED" => ("Visita confirmada", ""),
            "BUDGET" => ("Orçamento", ""),
            "Ordem de serviço" => ("Ordem de serviço", ""),
            _ => ("Pedido atualizado", "O status do pedido mudou"),
        };
        
        notification.title = title.to_string();
        notification.body = body.to_string();
        
        sqlx::query(
            "INSERT INTO notifications (user_id, title, body, read)
             VALUES ($1, $2, $3, false)"
        )
        .bind(Uuid::from_str(&notification.user_id).unwrap_or_default())
        .bind(notification.title.clone())
        .bind(notification.body)
        .execute(&*self.db.pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Problem to send notification: {}", e.to_string()).as_str()))?;
        
        self.ws_notification.send_to_user(notification.user_id, serde_json::to_value(notification.title).unwrap()).await;
        
        Ok(())
    }
    
    async fn visit_payment_done(&self, service_schedule: ServiceSchedule) -> Result<(), AppError> {
        
        
        Ok(())
    }
}