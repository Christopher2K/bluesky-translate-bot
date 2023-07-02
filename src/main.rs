use std::default;

use reqwest::blocking;
use serde::{Deserialize, Serialize};

const BSKY_SERVICE: &'static str = "https://bsky.social";

#[derive(Debug, Serialize)]
struct CreateSessionProperties {
    identifier: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionResponse {
    access_jwt: String,
    refresh_jwt: String,
    did: String,
    email: String,
    handle: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CreateRecordProperties<T> {
    repo: String,       // Better type for this, think about an enum maybe
    collection: String, // NSID
    #[serde(skip_serializing_if = "Option::is_none")]
    rkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<bool>,
    record: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_commit: Option<String>, // CID
}

// app.bsky.feed.post
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Post {
    text: String,
    created_at: String,
}

fn main() {
    let http_client = blocking::Client::default();
    let login_url = format!("{}/xrpc/com.atproto.server.createSession", BSKY_SERVICE);
    let login_data = CreateSessionProperties {
        identifier: "llcoolchris.dev".to_string(),
        password: "".to_string(),
    };

    let response = http_client
        .post(login_url)
        .json(&login_data)
        .send()
        .unwrap();
    let login_response_data = response.json::<CreateSessionResponse>().unwrap();
    println!("Authenticated");
    println!("Data: {:?}", &login_response_data);

    // Post something
    let create_post_url = format!("{}/xrpc/com.atproto.repo.createRecord", BSKY_SERVICE);
    let create_post_data: CreateRecordProperties<Post> = CreateRecordProperties {
        repo: login_response_data.did.to_string(),
        collection: "app.bsky.feed.post".to_string(),
        rkey: None,
        record: Post {
            text: "HELLO FROM RUST!".to_string(),
            created_at: "2023-07-02T21:21:36.422Z".to_string(),
        },
        validate: None,
        swap_commit: None,
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&create_post_data).unwrap()
    );

    let create_record_response = http_client
        .post(create_post_url)
        .json(&create_post_data)
        .header(
            "Authorization",
            format!("Bearer {}", login_response_data.access_jwt),
        )
        .send();

    match create_record_response {
        Ok(_response) => {
            println!("POSTED!");
            println!("{}", _response.json::<serde_json::Value>().unwrap());
        }
        Err(_error) => {
            println!("Not Posted!");
            println!("{}", _error);
        }
    }
}
