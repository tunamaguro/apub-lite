use crate::persistence::http_client::HttpClient;
use apub_activitypub::activity_json::APPLICATION_ACTIVITY_JSON;
use apub_kernel::{model::activity::SendActivity, repository::activity::ActivityRepository};
use axum::http::{header, HeaderMap, HeaderName, HeaderValue};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[async_trait::async_trait]
impl ActivityRepository for HttpClient {
    async fn send_create_activity<T: Serialize + Sync>(
        &self,
        event: &SendActivity<T>,
    ) -> anyhow::Result<()> {
        let body = serde_json::to_vec(&event.activity)?;

        // Mastdon needs digest
        // https://docs.joinmastodon.org/spec/security/#digest
        let digest = Sha256::digest(&body);
        let digest_64 = format!("SHA-256={}", BASE64_STANDARD.encode(digest));

        let date = httpdate::fmt_http_date(std::time::SystemTime::now());
        let signed_string = format!(
            "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}",
            event.inbox.path(),
            event.inbox.host(),
            date,
            digest_64
        );

        let signature = event.signer.sign(signed_string.as_bytes());
        let signature_64 = BASE64_STANDARD.encode(signature);

        let signature_header = format!(
            r#"keyId="{}",algorithm="rsa-sha256",headers"(request-target) host date digest",signature="{}""#,
            event.key_uri, signature_64
        );

        let headers = HeaderMap::from_iter([
            (header::CONTENT_TYPE, APPLICATION_ACTIVITY_JSON),
            (header::ACCEPT, APPLICATION_ACTIVITY_JSON),
            (
                header::HOST,
                HeaderValue::from_str(event.inbox.host()).unwrap(),
            ),
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

        let res = self
            .inner_ref()
            .post(event.inbox.to_string())
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let res_text = res.text().await?;
        tracing::info!(res_text);

        Ok(())
    }
}
