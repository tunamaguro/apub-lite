use apub_activitypub::{core::actor::Actor as _, model::person::Person, webfinger::AcctUri};
use apub_shared::model::{id::Id, resource_url::ResourceUrl};
use typed_builder::TypedBuilder;

use crate::user::model::UserId;

pub type ActorId = Id<Actor>;

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Actor {
    pub actor_id: ActorId,
    pub actor_url: ResourceUrl,
    preferred_name: String,
    display_name: Option<String>,
    inbox: ResourceUrl,
    shared_inbox: Option<ResourceUrl>,
    local_id: Option<UserId>,
}

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CreateActorEvent {
    pub preferred_name: String,
    pub actor_url: ResourceUrl,
    pub display_name: Option<String>,
    pub inbox: ResourceUrl,
    #[builder(default = None)]
    pub shared_inbox: Option<ResourceUrl>,
    #[builder(default)]
    pub local_id: Option<UserId>,
}

impl From<CreateActorEvent> for Actor {
    fn from(value: CreateActorEvent) -> Self {
        let actor_id = ActorId::new();

        let CreateActorEvent {
            actor_url,
            preferred_name,
            display_name,
            inbox,
            shared_inbox,
            local_id,
            ..
        } = value;

        Actor::builder()
            .actor_id(actor_id)
            .actor_url(actor_url)
            .preferred_name(preferred_name)
            .display_name(display_name)
            .inbox(inbox)
            .shared_inbox(shared_inbox)
            .local_id(local_id)
            .build()
    }
}

impl From<Person> for CreateActorEvent {
    fn from(value: Person) -> Self {
        let actor_url = value.id().clone();
        let inbox = value.inbox().clone();
        let name = value.username().to_owned();
        CreateActorEvent::builder()
            .actor_url(actor_url)
            .inbox(inbox)
            .display_name(None)
            .preferred_name(name)
            .build()
    }
}

#[async_trait::async_trait]
pub trait ActorRepository: Send + Sync {
    async fn find_by_id(&self, actor_id: &ActorId) -> anyhow::Result<Actor>;
    async fn find_by_acct(&self, acct: &AcctUri) -> anyhow::Result<Actor>;
    async fn find_by_url(&self, actor_url: &ResourceUrl) -> anyhow::Result<Actor>;
    async fn create(&self, event: CreateActorEvent) -> anyhow::Result<Actor>;
}
