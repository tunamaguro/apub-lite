use apub_activitypub::{
    core::actor::Actor as _,
    model::{
        collection::{OrderedCollection, OrderedCollectionBase},
        context::Context,
        key::PublicKeyPem,
        person::{Person, SecurityPerson},
    },
    shared::activity_json::ActivityJson,
};
use apub_kernel::{prelude::*, rsa_key::model::RsaVerifyingKey};
use apub_registry::AppRegistryExt;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("User not found")]
    NotFound,
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for PersonError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PersonError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            PersonError::Internal(e) => {
                tracing::error!(error = %e);
                (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
            }
        }
    }
}

pub async fn person_handler(
    username: &str,
    registry: &impl AppRegistryExt,
) -> Result<impl IntoResponse, PersonError> {
    let user = registry.user_service().find_by_name(username).await?;

    let public_key = registry
        .rsa_key_repository()
        .find_public_key(&user.id)
        .await?;

    let config = registry.config();
    let user_key_id = user.user_key_uri::<RsaVerifyingKey>(&config);

    let person = Person::builder()
        .id(user.user_uri(&config))
        .preferred_username(user.name.clone())
        .inbox(user.inbox_uri(&config))
        .context(Context::activity_context_url().clone().into())
        .followers(user.followers_uri(&config))
        .kind(Default::default())
        .build();

    let person_id = person.id().clone();

    let public_key_pem = PublicKeyPem::builder()
        .public_key_pem(public_key.to_pkcs8()?)
        .id(user_key_id)
        .owner(person_id)
        .build();

    let security = SecurityPerson::builder()
        .inner(person)
        .public_key(public_key_pem)
        .build();

    tracing::info!(message = "Return person", name = username);

    Ok(ActivityJson(security))
}

pub async fn followers_handler(
    username: &str,
    registry: &impl AppRegistryExt,
) -> Result<impl IntoResponse, PersonError> {
    let user = registry.user_service().find_by_name(username).await?;

    let config = registry.config();

    let followers = registry
        .follower_repository()
        .find_followee(&user.id)
        .await?;

    let follower_url = followers
        .into_iter()
        .map(|v| v.actor_url)
        .collect::<Vec<_>>();

    let follower_collection = OrderedCollectionBase::builder()
        .total_items(follower_url.len())
        .ordered_items(follower_url)
        .build();
    let follower_collection = OrderedCollection::builder()
        .context(Context::activity_context_url().clone())
        .id(user.followers_uri(&config))
        .base(follower_collection)
        .build();

    Ok(ActivityJson(follower_collection))
}
