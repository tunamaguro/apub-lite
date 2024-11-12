use std::sync::Arc;

use apub_activitypub::model::acct_uri::AcctUri;
use apub_registry::AppRegistry;
use apub_shared::config::AppConfig;
use axum::{
    extract::{Query, State},
    http::header,
    response::IntoResponse,
    Json,
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
    State(config): State<Arc<AppConfig>>,
) -> Result<impl IntoResponse, WebFingerError> {
    let res = webfinger_handler(&query.resource, &registry, &config).await?;

    Ok(([(header::CONTENT_TYPE, "application/jrd+json")], Json(res)))
}
