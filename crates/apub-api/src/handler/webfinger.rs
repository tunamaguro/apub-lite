use apub_activitypub::{
    webfinger::AcctUri,
    webfinger::{WebFinger, WebFingerLink},
};
use apub_kernel::prelude::*;
use apub_registry::{AppRegistry, AppRegistryExt};
use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, thiserror::Error)]
pub enum WebFingerError {
    #[error("requested resource is not managed by this server")]
    OtherDomain,
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for WebFingerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            WebFingerError::OtherDomain => (StatusCode::NOT_FOUND, "").into_response(),
            WebFingerError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
        }
    }
}

pub async fn webfinger_handler(
    acct_uri: &AcctUri,
    registry: &AppRegistry,
) -> Result<impl IntoResponse, WebFingerError> {
    let config = registry.config();
    if config.host_uri().host() != acct_uri.host() {
        return Err(WebFingerError::OtherDomain);
    }
    let user = registry
        .user_repository()
        .find_by_name(acct_uri.user())
        .await?;

    let link = WebFingerLink::builder()
        .rel("self".into())
        .kind("application/activity+json".into())
        .href(user.user_uri(&config).into())
        .build();

    let w = WebFinger::builder()
        .subject(acct_uri.to_string())
        .aliases(vec![user.user_uri(&config).into()])
        .links(vec![link])
        .build();

    Ok(Json(w))
}
