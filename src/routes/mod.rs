use std::sync::Arc;

use axum::{
    Extension, Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use http::HeaderName;
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

use crate::{
    handlers::{
        auth::{login, register},
        message::{get_conversation, get_public_messages, send_message},
        user::{get_current_user, get_online_users, update_online_status},
        websocket::ws_handler,
    },
    middleware::auth::{AppState, auth_middleware},
};

pub fn create_routes(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
            HeaderName::from_static("accept"),
            HeaderName::from_static("origin"),
            HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true);

    let public_routes = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/ws", get(ws_handler))
        .with_state(state.clone());

    let protected_routes = Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/online", get(get_online_users))
        .route("/users/status", post(update_online_status))
        .route("/messages", post(send_message))
        .route("/messages/public", get(get_public_messages))
        .route("/messages/{receiver_id}", get(get_conversation))
        .route_layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state.clone());

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .layer(Extension(state))
}
