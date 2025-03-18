pub mod database;
pub mod jwt;

use std::env;
use dotenv::dotenv;

pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_HOST: &str = "127.0.0.1";

pub fn get_env_var(key: &str, default: &str) -> String {
    dotenv().ok();
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_port() -> u16 {
    get_env_var("PORT", &DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT)
}

pub fn get_host() -> String {
    get_env_var("HOST", DEFAULT_HOST)
}

pub fn get_database_url() -> String {
    get_env_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/chat_app",
    )
}

pub fn get_jwt_secret() -> String {
    get_env_var("JWT_SECRET", "super_secret_key_for_jwt_please_change_in_production")
}

pub fn get_jwt_expiration() -> i64 {
    get_env_var("JWT_EXPIRATION", "86400")
        .parse()
        .unwrap_or(86400)
} 