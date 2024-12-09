use std::str::FromStr;

use apub_kernel::activitypub::actor::Actor;
use apub_shared::model::resource_url::ResourceUrl;
use sqlx::types::Uuid;

pub struct ActorRow {
    pub actor_id: Uuid,
    pub actor_url: String,
    pub preferred_username: String,
    pub inbox_url: String,
    pub shared_inbox_url: Option<String>,
    pub local_user_id: Option<Uuid>,
}

impl TryFrom<ActorRow> for Actor {
    type Error = anyhow::Error;
    fn try_from(row: ActorRow) -> Result<Self, Self::Error> {
        let actor_url = ResourceUrl::from_str(&row.actor_url)?;
        let inbox_url = ResourceUrl::from_str(&row.inbox_url)?;
        let shared_inbox_url = row
            .shared_inbox_url
            .and_then(|v| ResourceUrl::from_str(&v).ok());

        let local_user_id = row.local_user_id.map(|v| v.into());
        let a = Actor::builder()
            .actor_id(row.actor_id)
            .actor_url(actor_url)
            .inbox(inbox_url)
            .preferred_name(row.preferred_username)
            .local_id(local_user_id)
            .shared_inbox(shared_inbox_url)
            .display_name(None)
            .build();
        Ok(a)
    }
}
