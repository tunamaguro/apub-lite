use crate::handler::person::{followers_handler, person_handler, PersonError};
use apub_activitypub::shared::activity_json::ACTIVITY_CONTENT_TYPE;
use apub_registry::AppRegistry;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

#[tracing::instrument(skip_all)]
pub async fn person(
    Path(username): Path<String>,
    State(registry): State<AppRegistry>,
) -> Result<impl IntoResponse, PersonError> {
    let res = person_handler(&username, &registry).await?;

    Ok(([ACTIVITY_CONTENT_TYPE], res))
}

#[tracing::instrument(skip_all)]
pub async fn followers(
    Path(username): Path<String>,
    State(registry): State<AppRegistry>,
) -> Result<impl IntoResponse, PersonError> {
    let res = followers_handler(&username, &registry).await?;

    Ok(([ACTIVITY_CONTENT_TYPE], res))
}
