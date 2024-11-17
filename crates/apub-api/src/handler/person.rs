use apub_activitypub::model::{
    actor::Actor,
    key::PublicKeyPem,
    person::{Person, SecurityPerson},
};
use apub_registry::{AppRegistry, AppRegistryExt};
use apub_shared::model::{id::UriId, resource_uri::ResourceUri};
use axum::{
    http::{uri, StatusCode},
    response::IntoResponse,
    Json,
};

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

fn person_id_to_rsa_id(uri: &UriId<Person>) -> ResourceUri {
    let key_id = uri::Builder::new()
        .scheme(uri.scheme().clone())
        .authority(uri.authority().unwrap().as_str())
        .path_and_query(format!("{}/rsa-key", uri.path_and_query().unwrap().path()))
        .build()
        .unwrap();

    key_id.try_into().unwrap()
}

pub async fn person_handler(
    username: &str,
    registry: &AppRegistry,
) -> Result<impl IntoResponse, PersonError> {
    let user = registry.user_repository().find_by_name(username).await?;

    let public_key = registry
        .rsa_key_repository()
        .find_public_key_or_generate(&user.id)
        .await?;

    let config = registry.config();

    let person = user.to_person(&config);
    let person_id = person.id().clone().into();
    let public_key_pem = PublicKeyPem::builder()
        .public_key_pem(public_key.to_pkcs8()?)
        .id(person_id_to_rsa_id(&person_id))
        .owner(person_id)
        .build();

    let security = SecurityPerson::builder()
        .person(person)
        .public_key(public_key_pem)
        .build();

    Ok(Json(security))
}
