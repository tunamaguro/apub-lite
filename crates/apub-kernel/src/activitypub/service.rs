use std::future::Future;

use apub_activitypub::{model::person::AnyActor, webfinger::AcctUri};
use apub_shared::model::resource_url::ResourceUrl;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    prelude::{ActivityRepository, RsaKeyRepository},
    rsa_key::model::RsaSingingKey,
};

use super::actor::{Actor, ActorRepository};

pub trait ActivityService: ActivityRepository {
    fn get_actor_by_url(&self, url: &ResourceUrl) -> impl Future<Output = anyhow::Result<Actor>>;
    fn get_actor_by_acct(&self, acct: &AcctUri) -> impl Future<Output = anyhow::Result<Actor>>;
}

pub struct ActivityServiceImpl<ActivityRepo, ActorRepo, KeyRepo> {
    activity: ActivityRepo,
    actor: ActorRepo,
    _rsa_key: KeyRepo,
}

impl<ActivityRepo, ActorRepo, KeyRepo> ActivityServiceImpl<ActivityRepo, ActorRepo, KeyRepo> {
    pub fn new(activity: ActivityRepo, actor: ActorRepo, rsa_key: KeyRepo) -> Self {
        Self {
            activity,
            actor,
            _rsa_key: rsa_key,
        }
    }
}

#[async_trait::async_trait]
impl<ActivityRepo, ActorRepo, KeyRepo> ActivityRepository
    for ActivityServiceImpl<ActivityRepo, ActorRepo, KeyRepo>
where
    ActivityRepo: ActivityRepository,
    ActorRepo: Send + Sync,
    KeyRepo: Send + Sync,
{
    async fn post_activity<T: Serialize + Sync>(
        &self,
        activity: &T,
        inbox: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<()> {
        let bind = self
            .activity
            .post_activity(activity, inbox, signer, key_uri)
            .await?;
        Ok(bind)
    }
    async fn get_activity<T: DeserializeOwned>(&self, req: &ResourceUrl) -> anyhow::Result<T> {
        let bind = self.activity.get_activity(req).await?;
        Ok(bind)
    }
    async fn get_activity_with_sign<T: DeserializeOwned>(
        &self,
        req: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<T> {
        let bind = self
            .activity
            .get_activity_with_sign(req, signer, key_uri)
            .await?;
        Ok(bind)
    }
}

impl<ActivityRepo, ActorRepo, KeyRepo> ActivityService
    for ActivityServiceImpl<ActivityRepo, ActorRepo, KeyRepo>
where
    ActivityRepo: ActivityRepository,
    ActorRepo: ActorRepository,
    KeyRepo: RsaKeyRepository,
{
    async fn get_actor_by_url(&self, url: &ResourceUrl) -> anyhow::Result<Actor> {
        let res = self.actor.find_by_url(url).await;
        if let Ok(actor) = res {
            return Ok(actor);
        }

        // 理想的にはここでアクターの公開鍵も取得してDBへ格納する
        let res = self.activity.get_activity::<AnyActor>(url).await?;
        let actor = self.actor.create(res.into()).await?;

        Ok(actor)
    }

    async fn get_actor_by_acct(&self, _acct: &AcctUri) -> anyhow::Result<Actor> {
        // resolve webfinger

        // then, send get request
        unimplemented!()
    }
}
