use apub_activitypub::model::{activity::Create, note::Note};
use apub_kernel::{
    model::rsa_key::RsaVerifyingKey,
    repository::activity::{generate_activity_uri, generate_note_uri},
};
use apub_registry::{AppRegistry, AppRegistryExt};
use apub_shared::model::resource_uri::ResourceUrl;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendNoteQuery {
    message: String,
    inbox: ResourceUrl,
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

#[tracing::instrument(skip_all)]
pub async fn send_note(
    Query(query): Query<SendNoteQuery>,
    State(registry): State<AppRegistry>,
) -> Result<(), SendNoteError> {
    let user_repo = registry.user_repository();
    let user = user_repo.find_by_name(&query.user).await?;

    let user_signing_key = registry
        .rsa_key_repository()
        .find_private_key_or_generate(&user.id)
        .await?;

    let config = registry.config();
    let note_uri = generate_note_uri(&config);

    let note = Note::builder()
        .id(note_uri.into())
        .content(format!("<p>{}</p>", query.message))
        .in_reply_to(
            "https://activitypub.academy/@brauta_orgleoss/113521016036162806"
                .parse::<ResourceUrl>()
                .unwrap()
                .into(),
        )
        .to("https://www.w3.org/ns/activitystreams#Public"
            .to_string()
            .into())
        .build();

    tracing::info!(note=?note);

    let create_uri = generate_activity_uri(&config);
    let create = Create::builder()
        .object(note)
        .actor(user.user_uri(&config).into())
        .context(
            "https://www.w3.org/ns/activitystreams"
                .parse::<ResourceUrl>()
                .unwrap()
                .into(),
        )
        .id(create_uri.into())
        .build();

    let activity_repo = registry.activity_repository();
    activity_repo
        .post_note(
            &create,
            &query.inbox,
            &user_signing_key,
            &user.user_key_uri::<RsaVerifyingKey>(&config),
        )
        .await?;

    Ok(())
}
