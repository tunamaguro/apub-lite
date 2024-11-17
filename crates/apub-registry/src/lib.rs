use std::sync::Arc;

use apub_adapter::persistence::postgres::PostgresDb;
use apub_kernel::repository::{rsa_key::RsaKeyRepository, user::UserRepository};
use apub_shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    user_repository: Arc<dyn UserRepository>,
    rsa_key_repository: Arc<dyn RsaKeyRepository>,
    config: Arc<AppConfig>,
}

impl AppRegistry {
    pub fn new_postgres(pool: PostgresDb, config: AppConfig) -> Self {
        AppRegistry {
            user_repository: Arc::new(pool.clone()),
            rsa_key_repository: Arc::new(pool.clone()),
            config: Arc::new(config),
        }
    }
}

impl AppRegistryExt for AppRegistry {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
    fn rsa_key_repository(&self) -> Arc<dyn RsaKeyRepository> {
        self.rsa_key_repository.clone()
    }

    fn config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}

pub trait AppRegistryExt {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn rsa_key_repository(&self) -> Arc<dyn RsaKeyRepository>;
    fn config(&self) -> Arc<AppConfig>;
}
