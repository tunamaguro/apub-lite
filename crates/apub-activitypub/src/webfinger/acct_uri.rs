use std::{fmt, str::FromStr};

use axum::http::uri::Authority;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// `acct` Uri Scheme
///
/// ```txt
/// acct:bob@example.com
///      ─┬─ ─────┬─────
///     user     host   
/// ```
///
/// See https://datatracker.ietf.org/doc/html/rfc7565
#[derive(Debug, PartialEq, Clone)]
pub struct AcctUri {
    host: String,
    user: String,
}

impl AcctUri {
    pub fn new(host: impl AsRef<str>, user: impl AsRef<str>) -> Result<Self, AcctUriError> {
        use axum::http::uri;
        let host =
            uri::Authority::from_str(host.as_ref()).map_err(|_| AcctUriError::InvalidHost)?;
        let user =
            uri::Authority::from_str(user.as_ref()).map_err(|_| AcctUriError::InvalidUser)?;

        Ok(Self {
            host: host.to_string(),
            user: user.to_string(),
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn user(&self) -> &str {
        &self.user
    }
}

impl fmt::Display for AcctUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "acct:{}@{}", self.user, self.host)
    }
}

impl FromStr for AcctUri {
    type Err = AcctUriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (user, host) = parse_acct_uri(s)?;
        Ok(Self {
            host: host.to_string(),
            user: user.to_string(),
        })
    }
}

impl Serialize for AcctUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct AcctUriVisitor;

impl Visitor<'_> for AcctUriVisitor {
    type Value = AcctUri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format of `acct:user@host`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AcctUri::from_str(v).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for AcctUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AcctUriVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AcctUriError {
    /// `acct:`から始まらない
    #[error("must start `acct:`")]
    InvalidScheme,
    /// `@`がない
    #[error("missing `@`")]
    MissingAtMark,
    /// `@`が1つ以上ある
    #[error("many `@` found")]
    ManyAtMark,
    /// 不正な`name`
    #[error("invalid user")]
    InvalidUser,
    /// 不正な`host`
    #[error("invalid host")]
    InvalidHost,
}

const ACCT_SCHEME: &str = "acct:";

/// `acct` URIを`user`と`host`に分割する
fn parse_acct_uri(s: &str) -> Result<(&str, &str), AcctUriError> {
    if !s.starts_with(ACCT_SCHEME) {
        return Err(AcctUriError::InvalidScheme);
    }

    let s = &s[ACCT_SCHEME.len()..];

    let mut ss = s.split("@");

    let maybe_name = ss.next().ok_or(AcctUriError::MissingAtMark)?;
    let maybe_host = ss.next().ok_or(AcctUriError::MissingAtMark)?;

    if ss.next().is_some() {
        return Err(AcctUriError::ManyAtMark);
    }

    Authority::from_str(maybe_name).map_err(|_| AcctUriError::InvalidUser)?;
    Authority::from_str(maybe_host).map_err(|_| AcctUriError::InvalidHost)?;

    Ok((maybe_name, maybe_host))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn valid_acct_uri() {
        let s = "acct:bob@example.com";
        let (user, host) = parse_acct_uri(s).unwrap();
        assert_eq!("bob", user);
        assert_eq!("example.com", host);
    }

    #[test]
    fn without_acct_scheme_acct_uri() {
        let s = "http:bob@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidScheme), res);
    }

    #[test]
    fn missing_at_mark_acct_uri() {
        let s = "acct:example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::MissingAtMark), res);
    }

    #[test]
    fn many_at_mark_acct_uri() {
        let s = "acct:bob@@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::ManyAtMark), res);

        let s = "acct:@bob@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::ManyAtMark), res);
    }

    #[test]
    fn invalid_user_acct_uri() {
        let s = "acct: bob@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidUser), res);

        let s = "acct:b/ob@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidUser), res);

        let s = "acct:@example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidUser), res);
    }

    #[test]
    fn invalid_host_acct_uri() {
        let s = "acct:bob@ example.com";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidHost), res);

        let s = "acct:bob@example.com/123";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidHost), res);

        let s = "acct:bob@";
        let res = parse_acct_uri(s);
        assert_eq!(Err(AcctUriError::InvalidHost), res);
    }

    #[test]
    fn serialize_acct_uri() {
        let original = "acct:bob@example.com";
        let acct = AcctUri::from_str(original).unwrap();

        let serialized = serde_json::to_string(&acct).unwrap();

        assert_eq!(serialized, format!(r#""{}""#, original));
    }

    #[test]
    fn deserialize_acct_uri() {
        let original = r#""acct:bob@example.com""#;

        let deserialized = serde_json::from_str::<AcctUri>(original).unwrap();

        let expected = AcctUri {
            host: "example.com".to_string(),
            user: "bob".to_string(),
        };

        assert_eq!(deserialized, expected);
    }
}
