use std::net::{Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing, Router};
use tokio::net::TcpListener;

use apub_lite::{
    registry::AppRegistry,
    route,
    shared::{AppConfig, AppState},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = apub_lite::tracing::init();

    bootstrap().await?;

    Ok(())
}

async fn bootstrap() -> anyhow::Result<()> {
    use tower::ServiceBuilder;
    use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};

    let state = init_state();
    let hosted_uri = state.config.host_uri().to_string();

    let app = Router::new()
        .route("/health", routing::get(health_check))
        .route(
            "/.well-known/webfinger",
            routing::get(route::webfinger::webfinger),
        )
        .layer(
            ServiceBuilder::new()
                .layer(NormalizePathLayer::trim_trailing_slash())
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(state);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server started at {addr}");
    tracing::info!("Server hosted at {hosted_uri}");

    axum::serve(listener, app)
        .await
        .map_err(anyhow::Error::from)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

fn init_state() -> AppState {
    let app_uri = std::env::var("APUB_LITE_URL").unwrap_or("http://example.com".to_string());
    let config = AppConfig::new(&app_uri);
    let registry = AppRegistry::new();

    AppState::new(config, registry)
}
