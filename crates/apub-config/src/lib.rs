use apub_shared::model::resource_url::ResourceUrl;

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

    pub fn shared_inbox(&self) -> ResourceUrl {
        self.host_uri.clone().set_path("/inbox").to_owned()
    }
}
