use std::sync::Arc;

use axum::{
    Extension,
    Json,
};

use crate::{
    middleware::auth::{AppState, AuthUser},
    models::{errors::AppError, user::{User, UserResponse}},
};

pub async fn get_online_users(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = User::get_online_users(&state.db).await?;
    Ok(Json(users))
}

pub async fn update_online_status(
    Extension(state): Extension<Arc<AppState>>,
    auth_user: AuthUser,
    Json(is_online): Json<bool>,
) -> Result<Json<UserResponse>, AppError> {
    let user = auth_user.0;
    
    user.update_online_status(is_online, &state.db).await?;
    
    let updated_user = User::find_by_id(user.id, &state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Pengguna tidak ditemukan".to_string()))?;
    
    Ok(Json(updated_user.into_response()))
}

pub async fn get_current_user(
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(auth_user.0.into_response()))
} 