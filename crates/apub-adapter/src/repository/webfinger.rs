use crate::persistence::http_client::HttpClient;
use apub_activitypub::shared::jrd::APPLICATION_JRD_JSON;
use apub_activitypub::webfinger::{AcctUri, WebFinger};

use apub_activitypub::webfinger::WebFingerResolver;
use reqwest::header::CONTENT_TYPE;

#[derive(Debug, thiserror::Error)]
pub enum WebFingerError {
    #[error("something went wrong when request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Deserialization error")]
    Deserialize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Scheme {
    HTTP,
    #[default]
    HTTPS,
}

impl AsRef<str> for Scheme {
    fn as_ref(&self) -> &str {
        match self {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
        }
    }
}

/// Resolve a WebFinger resource for the given account.
async fn resolve_webfinger_inner(
    url: &str,
    client: &reqwest::Client,
) -> Result<WebFinger, WebFingerError> {
    let res = client
        .get(url)
        .header(CONTENT_TYPE, APPLICATION_JRD_JSON)
        .send()
        .await?
        .json::<WebFinger>()
        .await
        .map_err(|_| WebFingerError::Deserialize)?;

    Ok(res)
}

fn construct_webfinger_url(acct: &AcctUri, schema: &Scheme) -> String {
    format!(
        "{}://{}/.well-known/webfinger?resource={}",
        schema.as_ref(),
        acct.host(),
        acct
    )
}

async fn resolve_webfinger(
    acct: &AcctUri,
    client: &reqwest::Client,
) -> Result<WebFinger, WebFingerError> {
    let https_url = construct_webfinger_url(acct, &Scheme::HTTPS);
    let https_webfinger = resolve_webfinger_inner(&https_url, client).await;
    let https_err = match https_webfinger {
        Ok(res) => return Ok(res),
        Err(err) => err,
    };

    let http_url = construct_webfinger_url(acct, &Scheme::HTTP);
    let http_webfinger = resolve_webfinger_inner(&http_url, client).await;

    http_webfinger.map_err(|_| https_err)
}

#[async_trait::async_trait]
impl WebFingerResolver for HttpClient {
    type Error = WebFingerError;
    async fn resolve_webfinger(&self, actor: &AcctUri) -> Result<WebFinger, Self::Error> {
        resolve_webfinger(actor, self.inner_ref()).await
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::sync::LazyLock;

    use super::*;
    static ACCT: LazyLock<AcctUri> = LazyLock::new(|| "acct:foo@example.com".parse().unwrap());

    #[test]
    fn test_construct_webfinger_url() {
        {
            let expected =
                "https://example.com/.well-known/webfinger?resource=acct:foo@example.com";

            let actual = construct_webfinger_url(&ACCT, &Scheme::HTTPS);

            assert_eq!(expected, actual)
        }

        {
            let expected = "http://example.com/.well-known/webfinger?resource=acct:foo@example.com";

            let actual = construct_webfinger_url(&ACCT, &Scheme::HTTP);

            assert_eq!(expected, actual)
        }
    }
}
