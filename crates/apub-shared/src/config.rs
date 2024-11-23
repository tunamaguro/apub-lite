use crate::model::resource_url::ResourceUrl;

pub struct AppConfig {
    host_uri: ResourceUrl,
}

impl AppConfig {
    pub fn new(host_uri: &str) -> Self {
        Self {
            host_uri: host_uri.parse().unwrap(),
        }
    }

    pub fn host_uri(&self) -> &ResourceUrl {
        &self.host_uri
    }
}
