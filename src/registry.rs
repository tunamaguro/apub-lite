use std::sync::Arc;

use crate::{adapter::user::InMemoryUserRepo, repository::users::UserRepository};

#[derive(Clone)]
pub struct AppRegistry {
    user_repository: Arc<dyn UserRepository>,
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRegistry {
    pub fn new() -> Self {
        AppRegistry {
            user_repository: Arc::new(InMemoryUserRepo::new()),
        }
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
}
