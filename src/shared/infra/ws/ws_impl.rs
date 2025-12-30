use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use axum::extract::ws::Message;

use crate::shared::application::ws::ws::Ws;

#[derive(Clone)]
pub struct WsHub {
    connections: Arc<Mutex<HashMap<String, tokio::sync::mpsc::Sender<Message>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Ws for WsHub {
    async fn connect(&self, user_id: String, tx: tokio::sync::mpsc::Sender<Message>) {
        self.connections.lock().await.insert(user_id, tx);
    }

    async fn disconnect(&self, user_id: &String) {
        self.connections.lock().await.remove(user_id);
    }

    async fn send_to_user(&self, user_id: String, payload: serde_json::Value) {
        if let Some(tx) = self.connections.lock().await.get(&user_id) {
            let _ = tx.send(Message::Text(payload.to_string().into())).await;
        }
    }
}
