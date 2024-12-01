use apub_activitypub::{
    core::actor::Actor as _,
    model::{key::PublicKeyPem, person::SecurityPerson},
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
            PersonError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
        }
    }
}

pub async fn person_handler(
    username: &str,
    registry: &impl AppRegistryExt,
) -> Result<impl IntoResponse, PersonError> {
    let user = registry.user_repository().find_by_name(username).await?;

    let public_key = registry
        .rsa_key_repository()
        .find_public_key_or_generate(&user.id)
        .await?;

    let config = registry.config();
    let user_key_id = user.user_key_uri::<RsaVerifyingKey>(&config);

    let person = user.to_person(&config);
    let person_id = person.id().clone();

    let public_key_pem = PublicKeyPem::builder()
        .public_key_pem(public_key.to_pkcs8()?)
        .id(user_key_id)
        .owner(person_id)
        .build();

    let security = SecurityPerson::builder()
        .person(person)
        .public_key(public_key_pem)
        .build();

    tracing::info!(message = "Return person", name = username);

    Ok(ActivityJson(security))
}
