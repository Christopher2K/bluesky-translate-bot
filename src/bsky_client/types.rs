use serde::{Deserialize, Serialize};

// AUTHENTICATION
#[derive(Debug, Serialize)]
pub struct CreateSessionProperties {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionResponse {
    pub access_jwt: String,
    pub refresh_jwt: String,
    pub did: String,
    pub email: String,
    pub handle: String,
}

// Data wrapper
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecordProperties<T> {
    pub repo: String,       // Better type for this, think about an enum maybe
    pub collection: String, // NSID
    pub record: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>, // CID
}

// Entities
// Repos: app.bsky.feed.post
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub text: String,
    pub created_at: String,
}
