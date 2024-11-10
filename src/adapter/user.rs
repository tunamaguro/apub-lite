use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{
    model::user::{CreateUser, User},
    repository::users::UserRepository,
};

#[derive(Clone, Default)]
pub(crate) struct InMemoryUserRepo {
    map: Arc<Mutex<BTreeMap<String, User>>>,
}

impl InMemoryUserRepo {
    pub(crate) fn new() -> Self {
        let mut map = BTreeMap::new();

        map.entry("alice".to_string()).or_insert(User {
            name: "alice".to_string(),
        });

        map.entry("bob".to_string()).or_insert(User {
            name: "bob".to_string(),
        });

        InMemoryUserRepo {
            map: Arc::new(Mutex::new(map)),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepo {
    #[tracing::instrument(skip(self))]
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User> {
        let map = self.map.lock().unwrap();
        match map.get(name).ok_or(anyhow::anyhow!("Not found")) {
            Ok(user) => Ok(user.clone()),
            Err(err) => Err(err),
        }
    }
    #[tracing::instrument(skip(self))]
    async fn create(&self, event: CreateUser) -> anyhow::Result<()> {
        let mut map = self.map.lock().unwrap();
        match map.insert(event.name.clone(), event.into()) {
            Some(_) => Err(anyhow::anyhow!("Cannot created")),
            None => Ok(()),
        }
    }
}
