use anyhow::anyhow;
use reqwest::blocking::Client as HttpClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::types;

const DEFAULT_USER_AGENT: &'static str = "ll_cool_bsky_client-v1.0.0_alpha";
const DEFAULT_BSKY_SERVICE: &'static str = "https://bsky.social";

const XRPC_COM_ATPROTO_SERVER_CREATE_SESSION: &'static str =
    "/xrpc/com.atproto.server.createSession";
const XRPC_COM_ATPROTO_REPO_CREATE_RECORD: &'static str = "/xrpc/com.atproto.repo.createRecord";

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
        let create_session_url = self.get_url(XRPC_COM_ATPROTO_SERVER_CREATE_SESSION);
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

    pub fn create_record<Record: Serialize, Response: DeserializeOwned>(
        &self,
        collection: String,
        record: Record,
    ) -> anyhow::Result<Response> {
        let session_data = self
            .session_data
            .as_ref()
            .ok_or(anyhow!("The client is not authenticated"))?;

        let create_record_url = self.get_url(XRPC_COM_ATPROTO_REPO_CREATE_RECORD);
        let create_record_data = types::CreateRecordProperties {
            repo: session_data.handle.clone(),
            collection,
            record,
            rkey: None,
            validate: Some(true),
            swap_commit: None,
        };

        self.http_client
            .post(&create_record_url)
            .header(
                "Authorization",
                format!("Bearer {}", &session_data.access_jwt),
            )
            .json(&create_record_data)
            .send()
            .and_then(|response| response.json::<Response>())
            .map_err(|error| anyhow::anyhow!(error))
    }
}
