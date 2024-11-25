use apub_activitypub::{shared::jrd::JRD_CONTENT_TYPE, webfinger::AcctUri};
use apub_registry::AppRegistry;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::handler::webfinger::{webfinger_handler, WebFingerError};

#[derive(Debug, Deserialize)]
pub struct WebFingerQuery {
    resource: AcctUri,
}

#[tracing::instrument(skip_all)]
pub async fn webfinger(
    Query(query): Query<WebFingerQuery>,
    State(registry): State<AppRegistry>,
) -> Result<impl IntoResponse, WebFingerError> {
    let res = webfinger_handler(&query.resource, &registry).await?;

    Ok(([JRD_CONTENT_TYPE], res))
}
