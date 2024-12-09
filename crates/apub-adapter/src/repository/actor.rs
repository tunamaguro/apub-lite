use apub_activitypub::webfinger::AcctUri;
use apub_kernel::activitypub::actor::{Actor, ActorId, ActorRepository, CreateActorEvent};
use apub_shared::model::resource_url::ResourceUrl;

use crate::persistence::postgres::PostgresDb;

#[async_trait::async_trait]
impl ActorRepository for PostgresDb {
    async fn find_by_id(&self, actor_id: &ActorId) -> anyhow::Result<Actor> {
        todo!()
    }
    async fn find_by_acct(&self, acct: &AcctUri) -> anyhow::Result<Actor> {
        todo!()
    }
    async fn find_by_url(&self, actor_url: &ResourceUrl) -> anyhow::Result<Actor> {
        todo!()
    }
    async fn create(&self, event: CreateActorEvent) -> anyhow::Result<Actor> {
        todo!()
    }
}
