use std::sync::Arc;

use apub_adapter::persistence::{http_client::HttpClient, postgres::PostgresDb};
use apub_config::AppConfig;
use apub_kernel::{
    follower::repository::FollowerRepository, note::repository::NoteRepository, prelude::*,
};

#[derive(Clone)]
pub struct AppRegistry {
    postgres: PostgresDb,
    http_client: HttpClient,
    config: Arc<AppConfig>,
}

impl AppRegistry {
    pub fn new_postgres(pool: PostgresDb, config: AppConfig) -> Self {
        AppRegistry {
            postgres: pool,
            http_client: HttpClient::new(),
            config: Arc::new(config),
        }
    }
}

impl AppRegistryExt for AppRegistry {
    type UserRepo = PostgresDb;
    type RsaRepo = PostgresDb;
    type FollowerRepo = PostgresDb;
    type NoteRepo = PostgresDb;
    type ActivityRepo = HttpClient;
    fn user_repository(&self) -> Self::UserRepo {
        self.postgres.clone()
    }
    fn rsa_key_repository(&self) -> Self::RsaRepo {
        self.postgres.clone()
    }

    fn activity_repository(&self) -> Self::ActivityRepo {
        self.http_client.clone()
    }

    fn follower_repository(&self) -> Self::FollowerRepo {
        self.postgres.clone()
    }

    fn note_repository(&self) -> Self::NoteRepo {
        self.postgres.clone()
    }

    fn config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}

pub trait AppRegistryExt: Send + Sync {
    type UserRepo: UserRepository;
    type RsaRepo: RsaKeyRepository;
    type FollowerRepo: FollowerRepository;
    type ActivityRepo: ActivityRepository;
    type NoteRepo: NoteRepository;
    fn user_repository(&self) -> Self::UserRepo;
    fn rsa_key_repository(&self) -> Self::RsaRepo;
    fn activity_repository(&self) -> Self::ActivityRepo;
    fn follower_repository(&self) -> Self::FollowerRepo;
    fn note_repository(&self) -> Self::NoteRepo;
    fn config(&self) -> Arc<AppConfig>;
}
