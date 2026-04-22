mod api;
mod db;
mod solana;
mod models;

use std::sync::Arc;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;
use crate::models::AppState;
use crate::solana::solana::SolanaClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rpc_url    = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://solana-validator:8899".to_string());
    let program_id = std::env::var("PROGRAM_ID")
        .unwrap_or_else(|_| "326Z4n9fXoHK97GeGEYBy2tAhaqvexd7fueNB9Vv4c3t".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL requerida");

    let mut cfg = Config::new();
    cfg.url = Some(database_url);
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("No se pudo crear el pool de Postgres");

    db::db::init(&pool).await.expect("Error inicializando DB");

    let solana = SolanaClient::new(&rpc_url, &program_id)
        .expect("No se pudo inicializar SolanaClient");

    let state = Arc::new(AppState { pool, solana });
    let app   = api::routes::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("🎟️  TicketChain corriendo en http://0.0.0.0:3001");
    info!("🔗 Solana RPC: {}", rpc_url);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}