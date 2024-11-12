use crate::model::resource_uri::ResourceUri;

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
