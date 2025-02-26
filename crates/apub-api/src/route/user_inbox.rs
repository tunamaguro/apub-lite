use crate::handler::inbox::{inbox_handler, InboxError, InboxKinds};
use apub_activitypub::shared::activity_json::ActivityJson;
use apub_registry::AppRegistry;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

#[tracing::instrument(skip_all)]
pub async fn user_inbox(
    Path(username): Path<String>,
    State(registry): State<AppRegistry>,
    ActivityJson(activity): ActivityJson<InboxKinds>,
) -> Result<impl IntoResponse, InboxError> {
    inbox_handler(&username, activity, &registry).await
}
