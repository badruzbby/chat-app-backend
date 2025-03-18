use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Option<Uuid>,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageRequest {
    pub receiver_id: Option<Uuid>,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct MessageResponse {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub receiver_id: Option<Uuid>,
    pub receiver_username: Option<String>,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(sender_id: Uuid, request: MessageRequest) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            sender_id,
            receiver_id: request.receiver_id,
            content: request.content,
            is_read: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(self, _pool: &PgPool) -> Result<Self> {
        #[cfg(not(debug_assertions))]
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (id, sender_id, receiver_id, content, is_read, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, sender_id, receiver_id, content, is_read, created_at, updated_at
            "#,
            self.id,
            self.sender_id,
            self.receiver_id,
            self.content,
            self.is_read,
            self.created_at,
            self.updated_at
        )
        .fetch_one(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let message = self;

        Ok(message)
    }

    pub async fn get_conversation(
        user1_id: Uuid,
        user2_id: Uuid,
        _limit: i64,
        _pool: &PgPool,
    ) -> Result<Vec<Message>> {
        #[cfg(not(debug_assertions))]
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, receiver_id, content, is_read, created_at, updated_at
            FROM messages
            WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
            ORDER BY created_at DESC
            LIMIT $3
            "#,
            user1_id,
            user2_id,
            limit
        )
        .fetch_all(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let messages = vec![
            Message {
                id: Uuid::new_v4(),
                sender_id: user1_id,
                receiver_id: Some(user2_id),
                content: "Halo, apa kabar?".to_string(),
                is_read: true,
                created_at: Utc::now() - chrono::Duration::minutes(10),
                updated_at: Utc::now() - chrono::Duration::minutes(10),
            },
            Message {
                id: Uuid::new_v4(),
                sender_id: user2_id,
                receiver_id: Some(user1_id),
                content: "Baik, terima kasih! Kamu?".to_string(),
                is_read: true,
                created_at: Utc::now() - chrono::Duration::minutes(5),
                updated_at: Utc::now() - chrono::Duration::minutes(5),
            },
        ];

        Ok(messages)
    }

    pub async fn get_public_messages(_limit: i64, _pool: &PgPool) -> Result<Vec<Message>> {
        #[cfg(not(debug_assertions))]
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, receiver_id, content, is_read, created_at, updated_at
            FROM messages
            WHERE receiver_id IS NULL
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let messages = vec![
            Message {
                id: Uuid::new_v4(),
                sender_id: Uuid::new_v4(),
                receiver_id: None,
                content: "Halo semua! Ini pesan publik.".to_string(),
                is_read: true,
                created_at: Utc::now() - chrono::Duration::minutes(15),
                updated_at: Utc::now() - chrono::Duration::minutes(15),
            },
            Message {
                id: Uuid::new_v4(),
                sender_id: Uuid::new_v4(),
                receiver_id: None,
                content: "Selamat datang di chat app!".to_string(),
                is_read: true,
                created_at: Utc::now() - chrono::Duration::minutes(30),
                updated_at: Utc::now() - chrono::Duration::minutes(30),
            },
        ];

        Ok(messages)
    }

    pub async fn mark_as_read(&self, _pool: &PgPool) -> Result<()> {
        #[cfg(not(debug_assertions))]
        sqlx::query!(
            r#"
            UPDATE messages
            SET is_read = true, updated_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            self.id
        )
        .execute(_pool)
        .await?;

        Ok(())
    }
}
