use std::net::{Ipv4Addr, SocketAddr};

use apub_adapter::persistence::postgres::PostgresDb;
use apub_config::AppConfig;
use apub_kernel::user::model::CreateUser;
use apub_registry::{AppRegistry, AppRegistryExt};
use axum::{http::StatusCode, routing, Router};
use tokio::net::TcpListener;

use apub_api::route::{person, send_note, user_inbox, webfinger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = apub_tracing::init();

    bootstrap().await?;

    Ok(())
}

async fn bootstrap() -> anyhow::Result<()> {
    use tower::ServiceBuilder;
    use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};

    let state = init_registry().await;
    let _ = seed_db(&state).await.inspect_err(|e| tracing::error!(?e));

    let hosted_uri = state.config().host_uri().to_string();

    let app = Router::new()
        .route("/health", routing::get(health_check))
        .route("/users/:username", routing::get(person::person))
        .route(
            "/users/:username/followers",
            routing::get(person::followers),
        )
        .route(
            "/users/:username/inbox",
            routing::post(user_inbox::user_inbox),
        )
        .route("/send-note", routing::get(send_note::send_note))
        .route("/.well-known/webfinger", routing::get(webfinger::webfinger))
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

async fn init_registry() -> AppRegistry {
    let app_uri = std::env::var("APUB_LITE_URL").unwrap_or("http://example.com".to_string());
    let config = AppConfig::new(&app_uri);
    let postgres_db =
        PostgresDb::connect("postgresql://postgres:5432/app?user=app&password=password")
            .await
            .unwrap();

    AppRegistry::new_postgres(postgres_db, config)
}

async fn seed_db(registry: &AppRegistry) -> anyhow::Result<()> {
    use apub_kernel::prelude::*;
    let user_repo = registry.user_service();

    user_repo
        .create(CreateUser {
            name: "alice".to_string(),
        })
        .await?;

    user_repo
        .create(CreateUser {
            name: "bob".to_string(),
        })
        .await?;

    Ok(())
}
