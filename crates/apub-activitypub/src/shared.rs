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

use axum::http::{header, HeaderName, HeaderValue};

pub mod jrd {
    use super::*;
    /// `webfinger` SHOULD be served with `application/jrd+json`
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-10.2
    pub const APPLICATION_JRD_JSON: HeaderValue = HeaderValue::from_static("application/jrd+json");

    pub const JRD_CONTENT_TYPE: (HeaderName, HeaderValue) =
        (header::CONTENT_TYPE, APPLICATION_JRD_JSON);
}

pub mod activity_json {
    use super::*;
    /// Activity Streams 2.0 mime type
    /// See https://www.w3.org/TR/activitystreams-core/#media-type
    pub const APPLICATION_ACTIVITY_JSON: HeaderValue =
        HeaderValue::from_static("application/activity+json");

    pub const ACTIVITY_CONTENT_TYPE: (HeaderName, HeaderValue) =
        (header::CONTENT_TYPE, APPLICATION_ACTIVITY_JSON);
}
