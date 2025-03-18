mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use std::sync::Arc;
use std::net::SocketAddr;

use anyhow::Result;
use tracing::{
    info,
    error,
};
use tokio::net::TcpListener;

use crate::config::{
    database::create_db_pool,
    get_host,
    get_port,
};
use crate::middleware::auth::AppState;
use crate::routes::create_routes;
use crate::utils::setup_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();
    info!("🚀 Memulai server aplikasi chat...");
    info!("📊 Menghubungkan ke database...");

    let db_pool = match create_db_pool().await {
        Ok(pool) => {
            info!("✅ Koneksi database berhasil");
            pool
        },
        Err(e) => {
            error!("❌ Gagal menghubungkan ke database: {}", e);
            return Err(e);
        }
    };

    let state = Arc::new(AppState {
        db: db_pool,
    });

    let app = create_routes(state);
    let host = get_host();
    let port = get_port();
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    info!("🌐 Server berjalan di: http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
