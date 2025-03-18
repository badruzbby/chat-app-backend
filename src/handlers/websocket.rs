use std::sync::Arc;

use axum::{
    extract::{
        Query, State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use dashmap::DashMap;
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    config::jwt::validate_token,
    middleware::auth::AppState,
    models::{
        message::{Message, MessageRequest, MessageResponse},
        user::User,
    },
};

static CONNECTIONS: Lazy<DashMap<Uuid, Sender<WebSocketMessage>>> = Lazy::new(DashMap::new);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    Text {
        content: String,
        receiver_id: Option<Uuid>,
    },
    UserStatus {
        user_id: Uuid,
        username: String,
        is_online: bool,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct WebSocketParams {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WebSocketParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let claims = match validate_token(&params.token) {
        Ok(claims) => claims,
        Err(_) => {
            return ws.on_upgrade(|_socket| async {
                error!("Token tidak valid");
            });
        }
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return ws.on_upgrade(|_socket| async {
                error!("ID pengguna tidak valid");
            });
        }
    };

    let user = match User::find_by_id(user_id, &state.db).await {
        Ok(Some(user)) => user,
        _ => {
            return ws.on_upgrade(|_socket| async {
                error!("Pengguna tidak ditemukan");
            });
        }
    };

    if let Err(e) = user.update_online_status(true, &state.db).await {
        error!("Error updating online status: {}", e);
    }

    info!("User {} connected (id: {})", user.username, user.id);

    broadcast_user_status(user.id, &user.username, true).await;

    ws.on_upgrade(move |socket| handle_socket(socket, user, state))
}

async fn handle_socket(socket: WebSocket, user: User, state: Arc<AppState>) {
    let (tx, rx) = mpsc::channel(100);

    CONNECTIONS.insert(user.id, tx);

    let (sender, receiver) = socket.split();

    let user_copy = user.clone();
    let state_copy = state.clone();

    tokio::spawn(handle_incoming(receiver, user_copy, state_copy));
    tokio::spawn(handle_outgoing(sender, rx, user.clone()));

    let user_id = user.id;
    let username = user.username.clone();
    let db = state.db.clone();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        CONNECTIONS.remove(&user_id);

        if let Err(e) = user.update_online_status(false, &db).await {
            error!("Error updating offline status: {}", e);
        }

        broadcast_user_status(user_id, &username, false).await;

        info!("User {} disconnected", username);
    });
}

async fn handle_incoming(mut receiver: SplitStream<WebSocket>, user: User, state: Arc<AppState>) {
    while let Some(result) = receiver.next().await {
        match result {
            Ok(msg) => {
                if let Err(e) = process_message(msg, &user, &state).await {
                    error!("Error processing message: {}", e);
                }
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }
    debug!(
        "Incoming message handler for user {} stopped",
        user.username
    );
}

async fn handle_outgoing(
    mut sender: SplitSink<WebSocket, WsMessage>,
    mut receiver: Receiver<WebSocketMessage>,
    user: User,
) {
    while let Some(message) = receiver.recv().await {
        let msg_str = match serde_json::to_string(&message) {
            Ok(s) => s,
            Err(e) => {
                error!("Error serializing message: {}", e);
                continue;
            }
        };

        if let Err(e) = sender.send(WsMessage::Text(msg_str.into())).await {
            error!("Error sending message to {}: {}", user.username, e);
            break;
        }
    }
    debug!(
        "Outgoing message handler for user {} stopped",
        user.username
    );
}

async fn process_message(
    msg: WsMessage,
    user: &User,
    state: &Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    match msg {
        WsMessage::Text(text) => {
            let ws_message: WebSocketMessage = serde_json::from_str(&text)?;

            match ws_message {
                WebSocketMessage::Text {
                    content,
                    receiver_id,
                } => {
                    let msg_request = MessageRequest {
                        content,
                        receiver_id,
                    };

                    let message = Message::new(user.id, msg_request.clone());

                    let saved_message = message.create(&state.db).await?;

                    let response = MessageResponse {
                        id: saved_message.id,
                        sender_id: user.id,
                        sender_username: user.username.clone(),
                        receiver_id: saved_message.receiver_id,
                        receiver_username: if let Some(receiver_id) = saved_message.receiver_id {
                            let receiver = User::find_by_id(receiver_id, &state.db).await?;
                            receiver.map(|r| r.username)
                        } else {
                            None
                        },
                        content: saved_message.content,
                        is_read: saved_message.is_read,
                        created_at: saved_message.created_at,
                    };

                    if let Some(receiver_id) = saved_message.receiver_id {
                        if let Some(receiver_tx) = CONNECTIONS.get(&receiver_id) {
                            let _ = receiver_tx
                                .send(WebSocketMessage::Text {
                                    content: serde_json::to_string(&response)?,
                                    receiver_id: Some(user.id),
                                })
                                .await;
                        }
                    } else {
                        for conn in CONNECTIONS.iter() {
                            if *conn.key() != user.id {
                                let _ = conn
                                    .value()
                                    .send(WebSocketMessage::Text {
                                        content: serde_json::to_string(&response)?,
                                        receiver_id: None,
                                    })
                                    .await;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        WsMessage::Close(_) => {}
        _ => {}
    }

    Ok(())
}

async fn broadcast_user_status(user_id: Uuid, username: &str, is_online: bool) {
    let status_message = WebSocketMessage::UserStatus {
        user_id,
        username: username.to_string(),
        is_online,
    };

    for conn in CONNECTIONS.iter() {
        if *conn.key() != user_id {
            let _ = conn.value().send(status_message.clone()).await;
        }
    }
}
