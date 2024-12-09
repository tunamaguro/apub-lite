use apub_activitypub::webfinger::AcctUri;
use apub_kernel::activitypub::actor::{Actor, ActorId, ActorRepository, CreateActorEvent};
use apub_shared::model::resource_url::ResourceUrl;

use crate::{model::actor::ActorRow, persistence::postgres::PostgresDb};

#[async_trait::async_trait]
impl ActorRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find_by_id(&self, actor_id: &ActorId) -> anyhow::Result<Actor> {
        let row = sqlx::query_as!(
            ActorRow,
            r#"
        SELECT 
            actor_id, actor_url, preferred_username, inbox_url, shared_inbox_url, local_user_id
        FROM
            actors
        WHERE
            actors.actor_id = $1
        "#,
            actor_id.as_ref()
        )
        .fetch_one(self.inner_ref())
        .await?;
        row.try_into()
    }
    async fn find_by_acct(&self, acct: &AcctUri) -> anyhow::Result<Actor> {
        let name = acct.user();
        let host = acct.host();
        let row = sqlx::query_as!(
            ActorRow,
            r#"
        SELECT 
            actor_id, actor_url, preferred_username, inbox_url, shared_inbox_url, local_user_id
        FROM
            actors
        WHERE
            actors.host = $1 AND actors.preferred_username = $2
        "#,
            name,
            host
        )
        .fetch_one(self.inner_ref())
        .await?;
        row.try_into()
    }
    async fn find_by_url(&self, actor_url: &ResourceUrl) -> anyhow::Result<Actor> {
        let row = sqlx::query_as!(
            ActorRow,
            r#"
        SELECT 
            actor_id, actor_url, preferred_username, inbox_url, shared_inbox_url, local_user_id
        FROM
            actors
        WHERE
            actors.actor_url = $1
        "#,
            actor_url.as_str()
        )
        .fetch_one(self.inner_ref())
        .await?;
        row.try_into()
    }
    async fn create(&self, event: CreateActorEvent) -> anyhow::Result<Actor> {
        let actor = Actor::from(event);
        let host = actor.actor_url.host();
        let shared_inbox = actor.shared_inbox.as_ref().map(|v| v.as_str());
        let local_id = actor.local_id.as_ref().map(|v| v.as_ref());
        sqlx::query!(
            r#"
            INSERT INTO actors
                (actor_id, actor_url, host, preferred_username, inbox_url, shared_inbox_url, local_user_id)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7)
            "#,
            actor.actor_id.as_ref(),
            actor.actor_url.as_str(),
            host,
            actor.preferred_name,
            actor.inbox.as_str(),
            shared_inbox,
            local_id
        ).execute(self.inner_ref()).await?;

        Ok(actor)
    }
}
