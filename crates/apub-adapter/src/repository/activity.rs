use crate::persistence::http_client::HttpClient;
use apub_activitypub::shared::activity_json::APPLICATION_ACTIVITY_JSON;
use apub_kernel::{activitypub::activity::ActivityRepository, rsa_key::model::RsaSingingKey};
use apub_shared::model::resource_url::ResourceUrl;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};

/// Activityを署名して送信する
async fn post_activity<T: Serialize>(
    client: &HttpClient,
    activity: &T,
    inbox: &ResourceUrl,
    signer: &RsaSingingKey,
    key_uri: &ResourceUrl,
) -> anyhow::Result<()> {
    let body = serde_json::to_vec(&activity)?;

    // Mastdon needs digest
    // https://docs.joinmastodon.org/spec/security/#digest
    let digest = Sha256::digest(&body);
    let digest_64 = format!("SHA-256={}", BASE64_STANDARD.encode(digest));

    let date = httpdate::fmt_http_date(std::time::SystemTime::now());
    let signed_string = format!(
        "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}",
        inbox.path(),
        inbox.host(),
        date,
        digest_64
    );

    let signature = signer.sign(signed_string.as_bytes());
    let signature_64 = BASE64_STANDARD.encode(signature);

    let signature_header = format!(
        r#"keyId="{}",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="{}""#,
        key_uri, signature_64
    );

    let headers = HeaderMap::from_iter([
        (header::CONTENT_TYPE, APPLICATION_ACTIVITY_JSON),
        (header::ACCEPT, APPLICATION_ACTIVITY_JSON),
        (header::HOST, HeaderValue::from_str(inbox.host()).unwrap()),
        (header::DATE, HeaderValue::from_str(&date).unwrap()),
        (
            HeaderName::from_static("signature"),
            HeaderValue::from_str(&signature_header).unwrap(),
        ),
        (
            HeaderName::from_static("digest"),
            HeaderValue::from_str(&digest_64).unwrap(),
        ),
    ]);

    let res = client
        .inner_ref()
        .post(inbox.to_string())
        .headers(headers)
        .body(body)
        .send()
        .await?;
    let res_status = res.status();

    let body = serde_json::to_string_pretty(&activity).unwrap();

    tracing::info!(
        method = "POST",
        body = body,
        req = inbox.as_str(),
        status_code = ?res_status
    );

    Ok(())
}

async fn get_activity<T: DeserializeOwned>(
    client: &HttpClient,
    req: &ResourceUrl,
) -> anyhow::Result<T> {
    let headers = HeaderMap::from_iter([(header::ACCEPT, APPLICATION_ACTIVITY_JSON)]);
    let res = client
        .inner_ref()
        .get(req.as_str())
        .headers(headers)
        .send()
        .await?;

    let d = res.json().await?;
    Ok(d)
}

/// 署名してGetリクエストする
async fn get_activity_with_sign<T: DeserializeOwned>(
    client: &HttpClient,
    req: &ResourceUrl,
    signer: &RsaSingingKey,
    key_uri: &ResourceUrl,
) -> anyhow::Result<T> {
    let date = httpdate::fmt_http_date(std::time::SystemTime::now());
    let signed_string = format!(
        "(request-target): get {}\nhost: {}\ndate: {}",
        req.path(),
        req.host(),
        date,
    );

    let signature = signer.sign(signed_string.as_bytes());
    let signature_64 = BASE64_STANDARD.encode(signature);

    let signature_header = format!(
        r#"keyId="{}",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="{}""#,
        key_uri, signature_64
    );

    let headers = HeaderMap::from_iter([
        (header::ACCEPT, APPLICATION_ACTIVITY_JSON),
        (header::HOST, HeaderValue::from_str(req.host()).unwrap()),
        (header::DATE, HeaderValue::from_str(&date).unwrap()),
        (
            HeaderName::from_static("signature"),
            HeaderValue::from_str(&signature_header).unwrap(),
        ),
    ]);

    let res = client
        .inner_ref()
        .get(req.as_str())
        .headers(headers)
        .send()
        .await?;

    let res_status = res.status();

    tracing::info!(
        method = "GET",
        req = req.as_str(),
        status_code = ?res_status
    );

    let d: T = res.json().await?;

    Ok(d)
}

#[async_trait::async_trait]
impl ActivityRepository for HttpClient {
    #[tracing::instrument(skip(self, activity, signer))]
    async fn post_activity<T: Serialize + Sync>(
        &self,
        activity: &T,
        inbox: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<()> {
        post_activity(self, activity, inbox, signer, key_uri).await
    }
    #[tracing::instrument(skip(self))]
    async fn get_activity<T: DeserializeOwned>(&self, req: &ResourceUrl) -> anyhow::Result<T> {
        get_activity(self, req).await
    }

    #[tracing::instrument(skip(self, signer))]
    async fn get_activity_with_sign<T: DeserializeOwned>(
        &self,
        req: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<T> {
        get_activity_with_sign(self, req, signer, key_uri).await
    }
}
