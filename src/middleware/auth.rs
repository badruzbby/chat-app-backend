use crate::{
    config::jwt::validate_token,
    models::{errors::AppError, user::User},
};
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = auth.token();

    let claims =
        validate_token(token).map_err(|_| AppError::Auth("Token tidak valid".to_string()))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Auth("Token tidak valid".to_string()))?;

    let user = User::find_by_id(user_id, &state.db)
        .await?
        .ok_or_else(|| AppError::Auth("Pengguna tidak ditemukan".to_string()))?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

pub struct AuthUser(pub User);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<User>()
            .cloned()
            .ok_or_else(|| AppError::Auth("Tidak terautentikasi".to_string()))?;

        Ok(AuthUser(user))
    }
}

pub struct AppState {
    pub db: PgPool,
}
