use std::sync::Arc;

use apub_config::AppConfig;
use apub_registry::AppRegistry;
use axum::extract::FromRef;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub registry: AppRegistry,
}

impl AppState {
    pub fn new(config: AppConfig, registry: AppRegistry) -> Self {
        Self {
            config: config.into(),
            registry,
        }
    }
}
