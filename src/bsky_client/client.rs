use reqwest::blocking::Client as HttpClient;

use super::types;

const DEFAULT_USER_AGENT: &'static str = "ll_cool_bsky_client-v1.0.0_alpha";
const DEFAULT_BSKY_SERVICE: &'static str = "https://bsky.social";

const XRPC_CREATE_SESSION: &'static str = "/xrpc/com.atproto.server.createSession";

#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    identifier: String,
    password: String,
    service: String,
    session_data: Option<types::CreateSessionResponse>,
}

impl Client {
    pub fn new(identifier: String, password: String) -> Self {
        // FIXME: No unwrap sir!
        let http_client = HttpClient::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();

        Self {
            http_client,
            identifier,
            password,
            service: DEFAULT_BSKY_SERVICE.to_string(),
            session_data: None,
        }
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}{}", &self.service, path)
    }

    pub fn create_session(&mut self) -> anyhow::Result<()> {
        let create_session_url = self.get_url(XRPC_CREATE_SESSION);
        let create_session_data = types::CreateSessionProperties {
            identifier: self.identifier.clone(),
            password: self.password.clone(),
        };

        self.http_client
            .post(&create_session_url)
            .json(&create_session_data)
            .send()
            .and_then(|response| response.json::<types::CreateSessionResponse>())
            .map(|response| self.session_data = Some(response))
            .map_err(|error| anyhow::anyhow!(error))
    }
}
