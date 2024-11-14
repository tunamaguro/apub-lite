use std::sync::Arc;

use apub_adapter::persistence::postgres::PostgresDb;
use apub_kernel::repository::user::UserRepository;
use apub_shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    user_repository: Arc<dyn UserRepository>,
    config: Arc<AppConfig>,
}

impl AppRegistry {
    pub fn new_postgres(pool: PostgresDb, config: AppConfig) -> Self {
        AppRegistry {
            user_repository: Arc::new(pool),
            config: Arc::new(config),
        }
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    pub fn config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}
