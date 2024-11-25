use super::acct_uri::AcctUri;
use super::webfinger_object::WebFinger;

#[async_trait::async_trait]
pub trait WebFingerResolver {
    type Error: std::error::Error + Send + Sync;
    /// Resolves a WebFinger resource for the given account.
    ///
    /// # Arguments
    /// * `account` - The account to resolve (e.g. "user@example.com")
    async fn resolve_webfinger(&self, actor: &AcctUri) -> Result<WebFinger, Self::Error>;
}
