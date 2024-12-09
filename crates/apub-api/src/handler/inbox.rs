use apub_activitypub::{
    core::actor::Actor,
    model::{
        activity::{Accept, Follow, UndoPersonFollow},
        person::Person,
    },
};
use apub_kernel::{
    activitypub::activity::generate_activity_uri, follower::repository::FollowerRepository,
    prelude::*, rsa_key::model::RsaVerifyingKey,
};
use apub_registry::AppRegistryExt;
use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum InboxError {
    #[error("User not found")]
    NotFound,
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for InboxError {
    fn into_response(self) -> axum::response::Response {
        match self {
            InboxError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            InboxError::Internal(e) => {
                tracing::error!(error = %e);
                (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InboxKinds {
    Follow(Follow<Person, Person>),
    UnFollow(UndoPersonFollow<Person>),
}

pub async fn inbox_handler(
    username: &str,
    kind: InboxKinds,
    registry: &impl AppRegistryExt,
) -> Result<impl IntoResponse, InboxError> {
    let user = registry
        .user_service()
        .find_by_name(username)
        .await
        .map_err(|_| InboxError::NotFound)?;

    let signing_key = registry
        .rsa_key_repository()
        .find_private_key(&user.id)
        .await?;

    let config = registry.config();
    let user_key_id = user.user_key_uri::<RsaVerifyingKey>(&config);

    let config = registry.config();
    let activity_repo = registry.activity_repository();
    match kind {
        InboxKinds::Follow(follow) => {
            let follow_person = activity_repo.get_activity::<Person>(&follow.actor).await?;

            let follower_repo = registry.follower_repository();

            follower_repo.create(&user.id, follow_person.id()).await?;

            let accept = Accept::builder()
                .actor(user.user_uri(&config))
                .id(generate_activity_uri(&config).into())
                .object(follow)
                .context(Default::default())
                .build();

            activity_repo
                .post_activity(&accept, follow_person.inbox(), &signing_key, &user_key_id)
                .await?;
            tracing::info!(kind = "Accept", actor = %follow_person.id(), object = user.name);
        }
        InboxKinds::UnFollow(undo) => {
            let actor = undo.object.actor;
            let follow_person = activity_repo.get_activity::<Person>(&actor).await?;
            let follower_repo = registry.follower_repository();

            follower_repo.delete(&user.id, follow_person.id()).await?;

            tracing::info!(kind = "Undo", actor = %follow_person.id(), object = user.name);
        }
    };

    Ok(StatusCode::ACCEPTED)
}
