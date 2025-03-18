use anyhow::Result;
use backend::config::jwt::{generate_token, validate_token, Claims};
use backend::models::user::{RegisterRequest, User};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

#[tokio::test]
async fn test_jwt_token_generation() -> Result<()> {
    // Membuat request registrasi
    let request = RegisterRequest {
        username: "jwt_test".to_string(),
        password: "password123".to_string(),
        email: Some("jwt@example.com".to_string()),
    };
    // Membuat user baru
    let user = User::new(request).await?;
    // Membuat token
    let token = generate_token(user.id)?;
    // Memastikan token tidak kosong
    assert!(!token.is_empty());
    // Memastikan token dapat divalidasi
    let claims = validate_token(&token)?;
    // Memeriksa user_id dalam claims
    assert_eq!(claims.sub, user.id.to_string());
    Ok(())
}

#[tokio::test]
async fn test_jwt_token_expiration() -> Result<()> {
    let user_id = "test-user-123";
    let secret = "test_secret".as_bytes();
    // Membuat header
    let header = Header::default();
    // Membuat claims dengan waktu kedaluwarsa yang sudah lewat
    let now = Utc::now();
    let expired_time = now - Duration::hours(1);
    // Membuat payload kustom dengan waktu kedaluwarsa yang sudah lewat
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expired_time.timestamp(),
        iat: (expired_time - Duration::hours(1)).timestamp(),
        iss: "test_chat_app".to_string(),
    };
    // Menandatangani token
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    // Memastikan validasi gagal karena token sudah kedaluwarsa
    let validation = Validation::default();
    let result = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &validation
    );
    
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_invalid_token_signature() -> Result<()> {
    // Membuat valid UUID
    let user_id = Uuid::new_v4();
    // Membuat token dengan fungsi normal
    let token = generate_token(user_id)?;
    // Membuat secret berbeda
    let different_secret = "different_secret".as_bytes();
    // Mencoba memvalidasi dengan secret yang berbeda
    let validation = Validation::default();
    let result = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(different_secret),
        &validation
    );
    // Memastikan validasi gagal
    assert!(result.is_err());
    Ok(())
} 