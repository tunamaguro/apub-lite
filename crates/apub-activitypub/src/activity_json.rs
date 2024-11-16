use axum::http::{header, HeaderName, HeaderValue};

/// Activity Streams 2.0 mime type
/// See https://www.w3.org/TR/activitystreams-core/#media-type
pub const APPLICATION_ACTIVITY_JSON: HeaderValue =
    HeaderValue::from_static("application/activity+json");

pub const ACTIVITY_CONTENT_TYPE: (HeaderName, HeaderValue) =
    (header::CONTENT_TYPE, APPLICATION_ACTIVITY_JSON);
