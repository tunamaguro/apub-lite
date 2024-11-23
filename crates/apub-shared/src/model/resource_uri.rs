use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

/// リソースを示す`Uri`
///
/// `http`もしくは`https`で始まり、ホストも存在することを保証する
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(try_from = "Url", into = "Url")]
pub struct ResourceUri(Url);

impl ResourceUri {
    /// `ResourceUri`は生成時にホストが存在することを確認しているので安全
    pub fn host(&self) -> &str {
        self.0.host_str().unwrap()
    }
    pub fn scheme(&self) -> &str {
        self.0.scheme()
    }

    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.0.set_path(path);
        self
    }

    pub fn set_query(&mut self, query: &str) -> &mut Self {
        self.0.set_query(query.into());
        self
    }

    pub fn clear_query(&mut self) -> &mut Self {
        self.0.set_query(None);
        self
    }

    pub fn set_fragment(&mut self, fragment: &str) -> &mut Self {
        self.0.set_fragment(fragment.into());
        self
    }

    pub fn clear_fragment(&mut self) -> &mut Self {
        self.0.set_fragment(None);
        self
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ResourceUriError {
    /// Urlとして不正
    #[error("invalid url")]
    InvalidUrl,
    /// `http`か`https`でない
    #[error("expected `http` or `https`")]
    InvalidSchema,
    /// schemaがありホストがないのは通常`InvalidUri`だが念のため存在している
    #[error("missing host")]
    MissingHost,
    /// Authorityにユーザ名やパスワードが含まれている
    #[error("invalid authority")]
    InvalidAuthority,
}

fn valid_resource_uri(url: Url) -> Result<ResourceUri, (ResourceUriError, Url)> {
    const HTTPS: &str = "https";
    const HTTP: &str = "http";

    match url.scheme() {
        HTTPS | HTTP => {}
        _ => return Err((ResourceUriError::InvalidSchema, url)),
    }

    if !url.has_host() {
        return Err((ResourceUriError::MissingHost, url));
    }

    let has_password = url.password().is_some();
    let has_username = !url.username().is_empty();
    if has_password || has_username {
        return Err((ResourceUriError::InvalidAuthority, url));
    }

    Ok(ResourceUri(url))
}

impl Deref for ResourceUri {
    type Target = Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Url> for ResourceUri {
    fn as_ref(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<Url> for ResourceUri {
    type Error = ResourceUriError;
    fn try_from(value: Url) -> Result<Self, Self::Error> {
        valid_resource_uri(value).map_err(|(err, _)| err)
    }
}

impl FromStr for ResourceUri {
    type Err = ResourceUriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::from_str(s).map_err(|_| ResourceUriError::InvalidUrl)?;
        url.try_into()
    }
}

impl From<ResourceUri> for Url {
    fn from(value: ResourceUri) -> Self {
        value.0
    }
}

impl std::fmt::Display for ResourceUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn valid_resource_uri() {
        let uri = "https://example.com";
        assert!(uri.parse::<ResourceUri>().is_ok());
    }

    #[rstest]
    #[test]
    #[case("https://", ResourceUriError::InvalidUrl)]
    #[case("/foo/bar", ResourceUriError::InvalidUrl)]
    #[case("s3://foo/bar", ResourceUriError::InvalidSchema)]
    #[case("https://foo:password@example.com", ResourceUriError::InvalidAuthority)]
    fn invalid_resource_uri(#[case] uri: &str, #[case] err: ResourceUriError) {
        assert_eq!(uri.parse::<ResourceUri>().unwrap_err(), err)
    }

    #[test]
    fn serialize_resource_uri() {
        let original = "https://example.com/foo/bar";
        let acct = ResourceUri::from_str(original).unwrap();

        let serialized = serde_json::to_string(&acct).unwrap();

        assert_eq!(serialized, format!(r#""{}""#, original));
    }

    #[test]
    fn deserialize_valid_resource_uri() {
        let original = r#""https://example.com/foo/bar""#;
        let deserialized = serde_json::from_str::<ResourceUri>(original).unwrap();

        let expected = "https://example.com/foo/bar".parse().unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    #[should_panic]
    fn deserialize_invalid_resource_uri() {
        let original = r#""ftp://example.com/foo/bar""#;
        serde_json::from_str::<ResourceUri>(original).unwrap();
    }
}
