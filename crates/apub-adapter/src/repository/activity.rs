use crate::persistence::http_client::HttpClient;
use apub_activitypub::{
    model::activity::CreatePersonNote, shared::activity_json::APPLICATION_ACTIVITY_JSON,
};
use apub_kernel::{model::rsa_key::RsaSingingKey, repository::activity::ActivityRepository};
use apub_shared::model::resource_url::ResourceUrl;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::Serialize;
use sha2::{Digest, Sha256};

/// Activityを署名して送信する
async fn post_activity<T: Serialize + Sync>(
    client: &HttpClient,
    activity: &T,
    inbox: &ResourceUrl,
    signer: &RsaSingingKey,
    key_uri: &ResourceUrl,
) -> anyhow::Result<()> {
    {
        let a = serde_json::to_string_pretty(activity).unwrap();
        tracing::debug!(a);
    }

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

    tracing::info!(signature = signature_header, headers = ?headers);

    let res = client
        .inner_ref()
        .post(inbox.to_string())
        .headers(headers)
        .body(body)
        .send()
        .await?;
    let res_status = res.status();
    let res_text = res.text().await?;
    tracing::info!(status_code = ?res_status, text = res_text);

    Ok(())
}

#[async_trait::async_trait]
impl ActivityRepository for HttpClient {
    #[tracing::instrument(skip(self, activity, signer))]
    async fn post_note(
        &self,
        activity: &CreatePersonNote,
        inbox: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<()> {
        post_activity(self, activity, inbox, signer, key_uri).await
    }
}
