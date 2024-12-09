use std::{future::Future, sync::Arc};

use apub_config::AppConfig;

use crate::{
    activitypub::actor::{ActorRepository, CreateActorEvent},
    rsa_key::{
        model::{RsaSingingKey, RsaVerifyingKey, SaveKeyPairEvent},
        repository::RsaKeyRepository,
    },
};

use super::{
    model::{CreateUser, User, UserId},
    repository::UserRepository,
};

pub trait UserService: Send + Sync {
    fn find_by_name(&self, name: &str) -> impl Future<Output = anyhow::Result<User>>;
    fn find_by_id(&self, id: &UserId) -> impl Future<Output = anyhow::Result<User>>;
    fn create(&self, event: CreateUser) -> impl Future<Output = anyhow::Result<User>>;
}

pub struct UserServiceImpl<UserRepo, ActorRepo, KeyRepo> {
    user: UserRepo,
    actor: ActorRepo,
    rsa_key: KeyRepo,
    config: Arc<AppConfig>,
}

impl<UserRepo, ActorRepo, KeyRepo> UserServiceImpl<UserRepo, ActorRepo, KeyRepo> {
    pub fn new(user: UserRepo, actor: ActorRepo, rsa_key: KeyRepo, config: Arc<AppConfig>) -> Self {
        Self {
            user,
            actor,
            rsa_key,
            config,
        }
    }
}

impl<UserRepo, ActorRepo, KeyRepo> UserService for UserServiceImpl<UserRepo, ActorRepo, KeyRepo>
where
    UserRepo: UserRepository,
    ActorRepo: ActorRepository,
    KeyRepo: RsaKeyRepository,
{
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User> {
        let bind = self.user.find_by_name(name).await?;
        Ok(bind)
    }

    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<User> {
        let bind = self.user.find_by_id(id).await?;
        Ok(bind)
    }

    async fn create(&self, event: CreateUser) -> anyhow::Result<User> {
        let user = self.user.create(event).await?;
        let create_actor = CreateActorEvent::builder()
            .actor_url(user.user_uri(&self.config))
            .display_name(user.name.clone())
            .preferred_name(user.name.clone())
            .inbox(user.inbox_uri(&self.config))
            .local_id(user.id.clone())
            .build();

        let actor = self.actor.create(create_actor).await?;
        let (pkey, skey) = generate_key_pair()?;

        let key_url = user.user_key_uri::<RsaVerifyingKey>(&self.config);

        let key_pair = SaveKeyPairEvent::builder()
            .actor_id(&actor.actor_id)
            .key_url(&key_url)
            .public_key(&pkey)
            .private_key(&skey)
            .build();

        self.rsa_key.save_key_pair(key_pair).await?;

        Ok(user)
    }
}

fn generate_key_pair() -> anyhow::Result<(RsaVerifyingKey, RsaSingingKey)> {
    let skey = RsaSingingKey::new()?;
    let pkey = skey.to_public_key();

    Ok((pkey, skey))
}
