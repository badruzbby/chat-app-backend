use std::sync::Arc;
use std::net::SocketAddr;
use anyhow::Result;
use tracing::{
    info,
    error,
};
use tokio::net::TcpListener;
use backend::{
    config::{
        get_host,
        get_port,
    },
    create_db_pool,
    middleware::auth::AppState,
    routes::create_routes,
    utils::setup_tracing,
};

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
