use reqwest::Client;

#[derive(Clone, Debug, Default)]
pub struct HttpClient(Client);

impl HttpClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn inner_ref(&self) -> &Client {
        &self.0
    }
}
