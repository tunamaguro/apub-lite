use std::{marker::PhantomData, ops::Deref, str::FromStr};

use axum::http::{uri::Scheme, Uri};
use serde::{Deserialize, Serialize};

/// リソースを示す`Uri`
///
/// `http`もしくは`https`で始まることを保証する
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ResourceUri(#[serde(with = "uri_serde")] Uri);

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ResourceUriError {
    /// Uriとして不正
    #[error("invalid uri")]
    InvalidUri,
    /// スキーマが存在しない
    #[error("missing schema")]
    MissingSchema,
    /// `http`か`https`でない
    #[error("expected `http` or `https`")]
    InvalidSchema,
    /// schemaがありホストがないのは通常`InvalidUri`だが念のため存在している
    #[error("missing host")]
    MissingHost,
}

fn valid_resource_uri(uri: &Uri) -> Result<(), ResourceUriError> {
    match uri.scheme() {
        Some(s) => {
            let is_http_or_https = *s == Scheme::HTTP || *s == Scheme::HTTPS;
            if !is_http_or_https {
                return Err(ResourceUriError::InvalidSchema);
            }
        }
        _ => return Err(ResourceUriError::MissingSchema),
    }

    match uri.host() {
        Some(_) => {}
        _ => return Err(ResourceUriError::MissingHost),
    }

    Ok(())
}

impl Deref for ResourceUri {
    type Target = Uri;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Uri> for ResourceUri {
    fn as_ref(&self) -> &Uri {
        &self.0
    }
}

impl TryFrom<Uri> for ResourceUri {
    type Error = ResourceUriError;

    fn try_from(uri: Uri) -> Result<Self, Self::Error> {
        match valid_resource_uri(&uri) {
            Ok(_) => Ok(ResourceUri(uri)),
            Err(e) => Err(e),
        }
    }
}

impl FromStr for ResourceUri {
    type Err = ResourceUriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = s.parse::<Uri>().map_err(|_| ResourceUriError::InvalidUri)?;
        uri.try_into()
    }
}

mod uri_serde {
    use super::valid_resource_uri;
    use axum::http::Uri;
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    pub(super) fn serialize<S>(uri: &Uri, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.collect_str(&uri)
    }

    struct ResourceUriVisitor;

    impl<'de> Visitor<'de> for ResourceUriVisitor {
        type Value = Uri;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("resource uri")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let uri = v.parse::<Uri>().map_err(|e| de::Error::custom(e))?;

            match valid_resource_uri(&uri) {
                Ok(_) => Ok(uri),
                Err(e) => Err(de::Error::custom(e)),
            }
        }
    }

    pub(super) fn deserialize<'de, D>(de: D) -> Result<Uri, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_str(ResourceUriVisitor)
    }
}

/// リソースにつけられた`Uri`
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UriId<T> {
    resource_uri: ResourceUri,
    _marker: PhantomData<T>,
}

impl<T> From<ResourceUri> for UriId<T> {
    fn from(uri: ResourceUri) -> Self {
        Self {
            resource_uri: uri,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_resource_uri() {
        let uri = "https://example.com";
        assert!(uri.parse::<ResourceUri>().is_ok());
    }

    #[test]
    fn missing_host_uri() {
        let uri = "https://";
        assert_eq!(
            uri.parse::<ResourceUri>().unwrap_err(),
            ResourceUriError::InvalidUri
        );
    }

    #[test]
    fn missing_schema_uri() {
        let uri = "/foo/bar";
        assert_eq!(
            uri.parse::<ResourceUri>().unwrap_err(),
            ResourceUriError::MissingSchema
        );
    }

    #[test]
    fn invalid_schema_uri() {
        let uri = "s3://foo/bar";
        assert_eq!(
            uri.parse::<ResourceUri>().unwrap_err(),
            ResourceUriError::InvalidSchema
        );
    }
}
