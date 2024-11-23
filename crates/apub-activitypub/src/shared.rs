use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMany<T> {
    Many(Vec<T>),
    Single(T),
}

impl<T> From<T> for SingleOrMany<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<Vec<T>> for SingleOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

pub mod jrd {
    use axum::http::{header, HeaderName, HeaderValue};
    /// `webfinger` SHOULD be served with `application/jrd+json`
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-10.2
    pub const APPLICATION_JRD_JSON: HeaderValue = HeaderValue::from_static("application/jrd+json");

    pub const JRD_CONTENT_TYPE: (HeaderName, HeaderValue) =
        (header::CONTENT_TYPE, APPLICATION_JRD_JSON);
}
