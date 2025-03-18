// Mengekspos modul-modul untuk testing
pub mod app;
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

// Re-eksport type dan fungsi yang sering digunakan
pub use config::database::create_db_pool;
pub use config::jwt::{Claims, generate_token as create_token, validate_token};
pub use middleware::auth::{AppState, AuthUser};
