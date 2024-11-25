use apub_shared::model::id::UrlId;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrlOrObj<T> {
    Url(UrlId<T>),
    Obj(T),
}

impl<T> From<T> for UrlOrObj<T> {
    fn from(value: T) -> Self {
        Self::Obj(value)
    }
}

impl<T> From<UrlId<T>> for UrlOrObj<T> {
    fn from(value: UrlId<T>) -> Self {
        Self::Url(value)
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
    use axum::{
        body::Bytes,
        extract::{
            rejection::{BytesRejection, JsonDataError, JsonRejection, JsonSyntaxError},
            FromRequest, Request,
        },
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use header::HeaderMap;
    use serde::de::DeserializeOwned;

    use super::*;
    /// Activity Streams 2.0 mime type
    ///
    /// See https://www.w3.org/TR/activitystreams-core/#media-type
    pub const APPLICATION_ACTIVITY_JSON: HeaderValue =
        HeaderValue::from_static("application/activity+json");

    pub const ACTIVITY_CONTENT_TYPE: (HeaderName, HeaderValue) =
        (header::CONTENT_TYPE, APPLICATION_ACTIVITY_JSON);

    pub enum ActivityJsonRejection {
        BytesRejection(BytesRejection),
        JsonDataError(JsonDataError),
        JsonSyntaxError(JsonSyntaxError),
        MissingActivityContentType,
    }
    impl From<BytesRejection> for ActivityJsonRejection {
        fn from(value: BytesRejection) -> Self {
            ActivityJsonRejection::BytesRejection(value)
        }
    }

    impl From<JsonRejection> for ActivityJsonRejection {
        fn from(value: JsonRejection) -> Self {
            match value {
                JsonRejection::BytesRejection(rej) => ActivityJsonRejection::BytesRejection(rej),
                JsonRejection::JsonDataError(err) => ActivityJsonRejection::JsonDataError(err),
                JsonRejection::JsonSyntaxError(err) => ActivityJsonRejection::JsonSyntaxError(err),
                JsonRejection::MissingJsonContentType(_) => {
                    ActivityJsonRejection::MissingActivityContentType
                }
                _ => ActivityJsonRejection::MissingActivityContentType,
            }
        }
    }

    impl From<JsonSyntaxError> for ActivityJsonRejection {
        fn from(value: JsonSyntaxError) -> Self {
            ActivityJsonRejection::JsonSyntaxError(value)
        }
    }

    impl IntoResponse for ActivityJsonRejection {
        fn into_response(self) -> axum::response::Response {
            match self {
                ActivityJsonRejection::BytesRejection(rej) => rej.into_response(),
                ActivityJsonRejection::JsonDataError(err)=>err.into_response(),
                ActivityJsonRejection::JsonSyntaxError(err)=>err.into_response(),
                ActivityJsonRejection::MissingActivityContentType=>(StatusCode::UNSUPPORTED_MEDIA_TYPE,r#"Expected Content-Type `application/activity+json` or `application/ld+json; profile="https://www.w3.org/ns/activitystreams`"#).into_response()
            }
        }
    }

    pub struct ActivityJson<T>(pub T);

    pub(crate) fn activity_json_type(headers: &HeaderMap) -> bool {
        let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
            return false;
        };

        let Ok(content_type) = content_type.to_str() else {
            return false;
        };

        let Ok(mime) = content_type.parse::<mime::Mime>() else {
            return false;
        };

        // Allow `application/activity+json` and `application/ld+json; profile="https://www.w3.org/ns/activitystreams"`
        // See https://www.w3.org/TR/2018/REC-activitypub-20180123/#retrieving-objects
        let is_json = mime.type_() == "application" && mime.suffix().map_or(false, |s| s == "json");
        let is_activity = mime.subtype() == "activity";

        let is_activity_stream = mime.subtype() == "ld"
            && mime
                .get_param("profile")
                .map_or(false, |s| s == "https://www.w3.org/ns/activitystreams");

        is_json && (is_activity || is_activity_stream)
    }

    #[async_trait::async_trait]
    impl<T, S> FromRequest<S> for ActivityJson<T>
    where
        T: DeserializeOwned,
        S: Send + Sync,
    {
        type Rejection = ActivityJsonRejection;
        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            if activity_json_type(req.headers()) {
                let bytes = Bytes::from_request(req, state).await?;
                let v = Json::<T>::from_bytes(&bytes)?;
                Ok(ActivityJson(v.0))
            } else {
                Err(ActivityJsonRejection::MissingActivityContentType)
            }
        }
    }

    #[async_trait::async_trait]
    impl<T> IntoResponse for ActivityJson<T>
    where
        T: Serialize,
    {
        fn into_response(self) -> Response {
            match serde_json::to_vec(&self.0) {
                Ok(buf) => ([ACTIVITY_CONTENT_TYPE], buf).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )],
                    err.to_string(),
                )
                    .into_response(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{header, HeaderMap, HeaderValue};

    use super::activity_json::activity_json_type;

    #[test]
    fn test_valid_activity_json() {
        {
            let s = "application/activity+json";
            let headers =
                HeaderMap::from_iter([(header::CONTENT_TYPE, HeaderValue::from_str(s).unwrap())]);
            assert!(activity_json_type(&headers))
        }

        {
            let s = r#"application/ld+json; profile="https://www.w3.org/ns/activitystreams""#;
            let headers =
                HeaderMap::from_iter([(header::CONTENT_TYPE, HeaderValue::from_str(s).unwrap())]);
            assert!(activity_json_type(&headers))
        }
    }

    #[test]
    fn test_invalid_activity_json() {
        {
            let s = "application/json";
            let headers =
                HeaderMap::from_iter([(header::CONTENT_TYPE, HeaderValue::from_str(s).unwrap())]);
            assert!(!activity_json_type(&headers))
        }

        {
            let s = r#"application/ld+json"#;
            let headers =
                HeaderMap::from_iter([(header::CONTENT_TYPE, HeaderValue::from_str(s).unwrap())]);
            assert!(!activity_json_type(&headers))
        }
    }
}
