use std::net::{Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await?;

    Ok(())
}

async fn bootstrap() -> anyhow::Result<()> {
    let app = Router::new().route("/health", routing::get(health_check));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .map_err(anyhow::Error::from)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
