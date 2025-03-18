use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::middleware::auth::AppState;
use crate::handlers::{auth, user, message};

/// Buat aplikasi Axum untuk testing dan produksi
pub fn create_app(app_state: AppState) -> Router {
    let app_state = Arc::new(app_state);

    // Auth routes
    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    // User routes
    let user_routes = Router::new()
        .route("/me", get(user::get_current_user))
        .route("/online", get(user::get_online_users));

    // Message routes
    let message_routes = Router::new()
        .route("/", post(message::send_message))
        .route("/public", get(message::get_public_messages))
        .route("/:id", get(message::get_conversation));

    // Root router
    Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .nest("/messages", message_routes)
        .with_state(app_state)
} 