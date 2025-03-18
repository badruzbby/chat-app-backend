use std::sync::Arc;

use axum::{Extension, Json};

use crate::{
    config::jwt::generate_token,
    middleware::auth::AppState,
    models::{
        errors::AppError,
        user::{LoginRequest, RegisterRequest, TokenResponse, User},
    },
};

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    if request.username.is_empty() || request.password.is_empty() {
        return Err(AppError::Validation(
            "Username dan password wajib diisi".to_string(),
        ));
    }

    if User::find_by_username(&request.username, &state.db)
        .await?
        .is_some()
    {
        return Err(AppError::Validation("Username sudah digunakan".to_string()));
    }

    let user = User::new(request).await?;
    let user = user.create(&state.db).await?;
    let token = generate_token(user.id)?;
    let response = TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        user: user.into_response(),
    };

    Ok(Json(response))
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    if request.username.is_empty() || request.password.is_empty() {
        return Err(AppError::Validation(
            "Username dan password wajib diisi".to_string(),
        ));
    }

    let user = User::find_by_username(&request.username, &state.db)
        .await?
        .ok_or_else(|| AppError::Auth("Username atau password salah".to_string()))?;

    if !user.verify_password(&request.password)? {
        return Err(AppError::Auth("Username atau password salah".to_string()));
    }

    user.update_online_status(true, &state.db).await?;
    let token = generate_token(user.id)?;
    let response = TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        user: user.into_response(),
    };

    Ok(Json(response))
}
