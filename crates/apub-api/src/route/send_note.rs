use apub_activitypub::core::actor::Actor;
use apub_activitypub::model::person::Person;
use apub_activitypub::model::{activity::CreatePersonNote, context::Context, note::Note};
use apub_kernel::activitypub::model::{generate_activity_uri, generate_note_uri};
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
    let user_repo = registry.user_repository();
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

    let activity_repo = registry.activity_repository();
    let user_signing_key = registry
        .rsa_key_repository()
        .find_private_key_or_generate(&user.id)
        .await?;
    let user_key_id = user.user_key_uri::<RsaVerifyingKey>(&config);

    let followers = registry
        .follower_repository()
        .find_followee(&user.id)
        .await?;

    for v in followers {
        // 都度フォロワーに問い合わせて、inboxを取得する
        let follow_person = activity_repo
            .get_activity::<Person>(&v.actor_url, &user_signing_key, &user_key_id)
            .await?;

        activity_repo
            .post_note(
                &create,
                follow_person.inbox(),
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
