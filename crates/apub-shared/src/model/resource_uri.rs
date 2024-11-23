use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

/// リソースを示す`Url`
///
/// `http`もしくは`https`で始まり、ホストも存在することを保証する
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(try_from = "Url", into = "Url")]
pub struct ResourceUrl(Url);

impl ResourceUrl {
    /// `ResourceUrl`は生成時にホストが存在することを確認しているので安全
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
pub enum ResourceUrlError {
    /// Urlとして不正
    #[error("invalid url")]
    InvalidUrl,
    /// `http`か`https`でない
    #[error("expected `http` or `https`")]
    InvalidSchema,
    /// schemaがありホストがないのは通常`InvalidUrl`だが念のため存在している
    #[error("missing host")]
    MissingHost,
    /// Authorityにユーザ名やパスワードが含まれている
    #[error("invalid authority")]
    InvalidAuthority,
}

fn valid_resource_url(url: Url) -> Result<ResourceUrl, (ResourceUrlError, Url)> {
    const HTTPS: &str = "https";
    const HTTP: &str = "http";

    match url.scheme() {
        HTTPS | HTTP => {}
        _ => return Err((ResourceUrlError::InvalidSchema, url)),
    }

    if !url.has_host() {
        return Err((ResourceUrlError::MissingHost, url));
    }

    let has_password = url.password().is_some();
    let has_username = !url.username().is_empty();
    if has_password || has_username {
        return Err((ResourceUrlError::InvalidAuthority, url));
    }

    Ok(ResourceUrl(url))
}

impl Deref for ResourceUrl {
    type Target = Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Url> for ResourceUrl {
    fn as_ref(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<Url> for ResourceUrl {
    type Error = ResourceUrlError;
    fn try_from(value: Url) -> Result<Self, Self::Error> {
        valid_resource_url(value).map_err(|(err, _)| err)
    }
}

impl FromStr for ResourceUrl {
    type Err = ResourceUrlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::from_str(s).map_err(|_| ResourceUrlError::InvalidUrl)?;
        url.try_into()
    }
}

impl From<ResourceUrl> for Url {
    fn from(value: ResourceUrl) -> Self {
        value.0
    }
}

impl std::fmt::Display for ResourceUrl {
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
    fn valid_resource_url() {
        let url = "https://example.com";
        assert!(url.parse::<ResourceUrl>().is_ok());
    }

    #[rstest]
    #[test]
    #[case("https://", ResourceUrlError::InvalidUrl)]
    #[case("/foo/bar", ResourceUrlError::InvalidUrl)]
    #[case("s3://foo/bar", ResourceUrlError::InvalidSchema)]
    #[case("https://foo:password@example.com", ResourceUrlError::InvalidAuthority)]
    fn invalid_resource_url(#[case] url: &str, #[case] err: ResourceUrlError) {
        assert_eq!(url.parse::<ResourceUrl>().unwrap_err(), err)
    }

    #[test]
    fn serialize_resource_url() {
        let original = "https://example.com/foo/bar";
        let acct = ResourceUrl::from_str(original).unwrap();

        let serialized = serde_json::to_string(&acct).unwrap();

        assert_eq!(serialized, format!(r#""{}""#, original));
    }

    #[test]
    fn deserialize_valid_resource_url() {
        let original = r#""https://example.com/foo/bar""#;
        let deserialized = serde_json::from_str::<ResourceUrl>(original).unwrap();

        let expected = "https://example.com/foo/bar".parse().unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    #[should_panic]
    fn deserialize_invalid_resource_url() {
        let original = r#""ftp://example.com/foo/bar""#;
        serde_json::from_str::<ResourceUrl>(original).unwrap();
    }
}
