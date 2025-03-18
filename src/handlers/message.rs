use std::sync::Arc;

use axum::{
    extract::Path,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    middleware::auth::{AppState, AuthUser},
    models::{
        errors::AppError,
        message::{Message, MessageRequest, MessageResponse},
        user::User,
    },
};

pub async fn get_conversation(
    Extension(state): Extension<Arc<AppState>>,
    auth_user: AuthUser,
    Path(receiver_id): Path<Uuid>,
) -> Result<Json<Vec<MessageResponse>>, AppError> {
    let user_id = auth_user.0.id;
    
    let messages = Message::get_conversation(user_id, receiver_id, 50, &state.db).await?;
    
    let mut response_messages = Vec::new();
    for message in messages {
        let sender = User::find_by_id(message.sender_id, &state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Pengirim tidak ditemukan".to_string()))?;
        
        let receiver_user = if let Some(receiver_id) = message.receiver_id {
            let receiver = User::find_by_id(receiver_id, &state.db).await?;
            receiver.map(|r| r.username)
        } else {
            None
        };
        
        response_messages.push(MessageResponse {
            id: message.id,
            sender_id: message.sender_id,
            sender_username: sender.username,
            receiver_id: message.receiver_id,
            receiver_username: receiver_user,
            content: message.content,
            is_read: message.is_read,
            created_at: message.created_at,
        });
    }
    
    Ok(Json(response_messages))
}

pub async fn get_public_messages(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
) -> Result<Json<Vec<MessageResponse>>, AppError> {
    let messages = Message::get_public_messages(50, &state.db).await?;
    
    let mut response_messages = Vec::new();
    for message in messages {
        let sender = User::find_by_id(message.sender_id, &state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Pengirim tidak ditemukan".to_string()))?;
        
        response_messages.push(MessageResponse {
            id: message.id,
            sender_id: message.sender_id,
            sender_username: sender.username,
            receiver_id: None,
            receiver_username: None,
            content: message.content,
            is_read: message.is_read,
            created_at: message.created_at,
        });
    }
    
    Ok(Json(response_messages))
}

pub async fn send_message(
    Extension(state): Extension<Arc<AppState>>,
    auth_user: AuthUser,
    Json(request): Json<MessageRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let user = auth_user.0;
    
    if let Some(receiver_id) = request.receiver_id {
        let _receiver = User::find_by_id(receiver_id, &state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Penerima tidak ditemukan".to_string()))?;
    }
    
    let message = Message::new(user.id, request);
    
    let saved_message = message.create(&state.db).await?;
    
    let sender_username = user.username;
    
    let receiver_username = if let Some(receiver_id) = saved_message.receiver_id {
        let receiver = User::find_by_id(receiver_id, &state.db).await?;
        receiver.map(|r| r.username)
    } else {
        None
    };
    
    let response = MessageResponse {
        id: saved_message.id,
        sender_id: saved_message.sender_id,
        sender_username,
        receiver_id: saved_message.receiver_id,
        receiver_username,
        content: saved_message.content,
        is_read: saved_message.is_read,
        created_at: saved_message.created_at,
    };
    
    Ok(Json(response))
} 