use axum::extract::ws::{Message, WebSocket};

use crate::shared::application::ws::ws::Ws;

pub async fn handle_socket(
    mut socket: WebSocket,
    hub: Box<dyn Ws>,
    user_id: String
) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(32);

    hub.connect(user_id.clone(), tx).await;

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                if socket.send(msg).await.is_err() {
                    break;
                }
            }
            
            result = socket.recv() => {
                match result {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }

    hub.disconnect(&user_id).await;
}
