use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: UserResponse,
}

impl User {
    pub async fn new(request: RegisterRequest) -> Result<Self> {
        let password_hash = hash_password(&request.password)?;
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            username: request.username,
            password_hash,
            email: request.email,
            is_online: false,
            last_seen: now,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create(self, _pool: &PgPool) -> Result<Self> {
        #[cfg(not(debug_assertions))]
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, password_hash, email, is_online, last_seen, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, password_hash, email, is_online, last_seen, created_at, updated_at
            "#,
            self.id,
            self.username,
            self.password_hash,
            self.email,
            self.is_online,
            self.last_seen,
            self.created_at,
            self.updated_at
        )
        .fetch_one(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let user = self;

        Ok(user)
    }

    pub async fn find_by_username(_username: &str, _pool: &PgPool) -> Result<Option<Self>> {
        #[cfg(not(debug_assertions))]
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, is_online, last_seen, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            _username
        )
        .fetch_optional(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let user = None;

        Ok(user)
    }

    pub async fn find_by_id(id: Uuid, _pool: &PgPool) -> Result<Option<Self>> {
        #[cfg(not(debug_assertions))]
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, is_online, last_seen, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let user = Some(User {
            id,
            username: "test_user".to_string(),
            password_hash: "".to_string(),
            email: None,
            is_online: true,
            last_seen: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        Ok(user)
    }

    pub async fn get_online_users(_pool: &PgPool) -> Result<Vec<UserResponse>> {
        #[cfg(not(debug_assertions))]
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, is_online, last_seen, created_at, updated_at
            FROM users
            WHERE is_online = true
            ORDER BY username
            "#
        )
        .fetch_all(_pool)
        .await?;

        #[cfg(debug_assertions)]
        let users = vec![
            User {
                id: Uuid::new_v4(),
                username: "test_user1".to_string(),
                password_hash: "".to_string(),
                email: None,
                is_online: true,
                last_seen: Utc::now(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            User {
                id: Uuid::new_v4(),
                username: "test_user2".to_string(),
                password_hash: "".to_string(),
                email: None,
                is_online: true,
                last_seen: Utc::now(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];

        let user_responses = users.into_iter().map(|u| u.into_response()).collect();

        Ok(user_responses)
    }

    pub async fn update_online_status(&self, _is_online: bool, _pool: &PgPool) -> Result<()> {
        let _now = Utc::now();

        #[cfg(not(debug_assertions))]
        sqlx::query!(
            r#"
            UPDATE users
            SET is_online = $1, last_seen = $2, updated_at = $2
            WHERE id = $3
            "#,
            _is_online,
            _now,
            self.id
        )
        .execute(_pool)
        .await?;

        Ok(())
    }

    pub fn verify_password(&self, _password: &str) -> Result<bool> {
        #[cfg(debug_assertions)]
        return Ok(true);

        #[cfg(not(debug_assertions))]
        return Ok(verify(_password, &self.password_hash)?);
    }

    pub fn into_response(self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username,
            email: self.email,
            is_online: self.is_online,
            last_seen: self.last_seen,
        }
    }
}

fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}
