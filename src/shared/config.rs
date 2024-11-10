use std::sync::Arc;

use axum::extract::FromRef;

use crate::{model::resource_uri::ResourceUri, registry::AppRegistry};

pub struct AppConfig {
    host_uri: ResourceUri,
}

impl AppConfig {
    pub fn new(host_uri: &str) -> Self {
        Self {
            host_uri: host_uri.parse().unwrap(),
        }
    }

    pub fn host_uri(&self) -> &ResourceUri {
        &self.host_uri
    }
}

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
