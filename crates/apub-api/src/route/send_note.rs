use apub_activitypub::model::{activity::CreatePersonNote, context::Context, note::Note};
use apub_kernel::activitypub::activity::{generate_activity_uri, generate_note_uri};
use apub_kernel::prelude::*;
use apub_kernel::rsa_key::model::RsaVerifyingKey;
use apub_registry::{AppRegistry, AppRegistryExt};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendNoteQuery {
    message: String,
    user: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SendNoteError {
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for SendNoteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(e) => {
                tracing::error!(error=%e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

async fn send_note_handler(
    query: &SendNoteQuery,
    registry: impl AppRegistryExt,
) -> Result<impl IntoResponse, SendNoteError> {
    let user_repo = registry.user_service();
    let user = user_repo.find_by_name(&query.user).await?;

    let config = registry.config();
    let note_uri = generate_note_uri(&config);

    let note = Note::builder()
        .id(note_uri.into())
        .content(format!("<p>{}</p>", query.message))
        .attributed_to(user.user_uri(&config).into())
        .to(Note::public_address().clone().into())
        .build();

    tracing::info!(note=?note);

    let create_uri = generate_activity_uri(&config);
    let create = CreatePersonNote::builder()
        .object(note)
        .actor(user.user_uri(&config))
        .context(Context::activity_context_url().clone().into())
        .id(create_uri.into())
        .build();

    let activity_service = registry.activity_service();
    let user_signing_key = registry
        .rsa_key_repository()
        .find_private_key(&user.id)
        .await?;

    let followers = registry
        .follower_repository()
        .find_followee(&user.id)
        .await?;

    for f in followers {
        let follower_inbox = &f.inbox;

        activity_service
            .post_note(
                &create,
                follower_inbox,
                &user_signing_key,
                &user.user_key_uri::<RsaVerifyingKey>(&config),
            )
            .await?;
    }

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(skip_all)]
pub async fn send_note(
    Query(query): Query<SendNoteQuery>,
    State(registry): State<AppRegistry>,
) -> Result<impl IntoResponse, SendNoteError> {
    send_note_handler(&query, registry).await
}
