use apub_registry::AppRegistry;
use axum::{http::StatusCode, response::IntoResponse, Json};

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
    registry: &AppRegistry,
) -> Result<impl IntoResponse, PersonError> {
    let user = registry.user_repository().find_by_name(username).await?;

    let config = registry.config();

    let person = user.to_person(&config);

    Ok(Json(person))
}
