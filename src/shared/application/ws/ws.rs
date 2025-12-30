use async_trait::async_trait;
use axum::extract::ws::Message;

#[async_trait]
pub trait Ws: Send {
    async fn connect(&self, user_id: String, tx: tokio::sync::mpsc::Sender<Message>);
    async fn disconnect(&self, user_id: &String);
    async fn send_to_user(&self, user_id: String, payload: serde_json::Value);
}