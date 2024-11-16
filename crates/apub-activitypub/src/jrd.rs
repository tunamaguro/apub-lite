use axum::http::{header, HeaderName, HeaderValue};

/// `webfinger` SHOULD be served with `application/jrd+json`
/// See https://datatracker.ietf.org/doc/html/rfc7033#section-10.2
pub const APPLICATION_JRD_JSON: HeaderValue = HeaderValue::from_static("application/jrd+json");

pub const JRD_CONTENT_TYPE: (HeaderName, HeaderValue) =
    (header::CONTENT_TYPE, APPLICATION_JRD_JSON);
