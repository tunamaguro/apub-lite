use std::sync::Arc;

use apub_adapter::persistence::{http_client::HttpClient, postgres::PostgresDb};
use apub_config::AppConfig;
use apub_kernel::repository::{
    activity::ActivityRepository, rsa_key::RsaKeyRepository, user::UserRepository,
};

#[derive(Clone)]
pub struct AppRegistry {
    postgres: Arc<PostgresDb>,
    http_client: Arc<HttpClient>,
    config: Arc<AppConfig>,
}

impl AppRegistry {
    pub fn new_postgres(pool: PostgresDb, config: AppConfig) -> Self {
        AppRegistry {
            postgres: Arc::new(pool),
            http_client: Arc::new(HttpClient::new()),
            config: Arc::new(config),
        }
    }
}

impl AppRegistryExt for AppRegistry {
    type UserRepo = PostgresDb;
    type RsaRepo = PostgresDb;
    type ActivityRepo = HttpClient;
    fn user_repository(&self) -> Arc<Self::UserRepo> {
        self.postgres.clone()
    }
    fn rsa_key_repository(&self) -> Arc<Self::RsaRepo> {
        self.postgres.clone()
    }

    fn activity_repository(&self) -> Arc<Self::ActivityRepo> {
        self.http_client.clone()
    }

    fn config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}

pub trait AppRegistryExt {
    type UserRepo: UserRepository;
    type RsaRepo: RsaKeyRepository;
    type ActivityRepo: ActivityRepository;
    fn user_repository(&self) -> Arc<Self::UserRepo>;
    fn rsa_key_repository(&self) -> Arc<Self::RsaRepo>;
    fn activity_repository(&self) -> Arc<Self::ActivityRepo>;
    fn config(&self) -> Arc<AppConfig>;
}
