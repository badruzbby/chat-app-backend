use anyhow::Result;
use backend::models::message::{Message, MessageRequest, MessageResponse};
use backend::models::user::{RegisterRequest, User};

#[tokio::test]
async fn test_message_creation() -> Result<()> {
    // Membuat user untuk pengirim dan penerima
    let sender_request = RegisterRequest {
        username: "sender".to_string(),
        password: "password123".to_string(),
        email: None,
    };
    let receiver_request = RegisterRequest {
        username: "receiver".to_string(),
        password: "password456".to_string(),
        email: None,
    };

    let sender = User::new(sender_request).await?;
    let receiver = User::new(receiver_request).await?;

    // Membuat request pesan
    let message_request = MessageRequest {
        receiver_id: Some(receiver.id),
        content: "Ini adalah pesan test".to_string(),
    };

    // Membuat pesan baru
    let message = Message::new(sender.id, message_request);

    // Verifikasi data pesan
    assert_eq!(message.sender_id, sender.id);
    assert_eq!(message.receiver_id, Some(receiver.id));
    assert_eq!(message.content, "Ini adalah pesan test");
    assert_eq!(message.is_read, false);

    Ok(())
}

#[tokio::test]
async fn test_public_message() -> Result<()> {
    // Membuat user untuk pengirim
    let sender_request = RegisterRequest {
        username: "public_sender".to_string(),
        password: "password789".to_string(),
        email: None,
    };

    let sender = User::new(sender_request).await?;
    // Membuat request pesan publik
    let message_request = MessageRequest {
        receiver_id: None,
        content: "Ini adalah pesan publik".to_string(),
    };
    // Membuat pesan baru
    let message = Message::new(sender.id, message_request);
    // Verifikasi data pesan
    assert_eq!(message.sender_id, sender.id);
    assert_eq!(message.receiver_id, None);
    assert_eq!(message.content, "Ini adalah pesan publik");
    assert_eq!(message.is_read, false);

    Ok(())
}

#[tokio::test]
async fn test_message_response() -> Result<()> {
    // Membuat user untuk pengirim dan penerima
    let sender_request = RegisterRequest {
        username: "resp_sender".to_string(),
        password: "password123".to_string(),
        email: None,
    };
    let receiver_request = RegisterRequest {
        username: "resp_receiver".to_string(),
        password: "password456".to_string(),
        email: None,
    };

    let sender = User::new(sender_request).await?;
    let receiver = User::new(receiver_request).await?;
    // Membuat request pesan
    let message_request = MessageRequest {
        receiver_id: Some(receiver.id),
        content: "Pesan untuk response test".to_string(),
    };
    // Membuat pesan
    let message = Message::new(sender.id, message_request);
    // Membuat response manual untuk testing
    let response = MessageResponse {
        id: message.id,
        sender_id: message.sender_id,
        sender_username: sender.username.clone(),
        receiver_id: message.receiver_id,
        receiver_username: Some(receiver.username.clone()),
        content: message.content.clone(),
        is_read: message.is_read,
        created_at: message.created_at,
    };
    // Verifikasi data response
    assert_eq!(response.id, message.id);
    assert_eq!(response.sender_id, message.sender_id);
    assert_eq!(response.receiver_id, message.receiver_id);
    assert_eq!(response.content, message.content);
    assert_eq!(response.is_read, message.is_read);

    Ok(())
}
