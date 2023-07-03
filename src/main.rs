mod bsky_client;
mod config;

use bsky_client::client::Client;
use bsky_client::types::Post;
use config::{AppConfig, AppConfigVariableName};

fn main() {
    AppConfig::load().expect("Cannot initialize app config");

    let mut client = Client::new(
        AppConfig::get(AppConfigVariableName::BskyIdentifier),
        AppConfig::get(AppConfigVariableName::BskyPassword),
    );
    let create_session_result = client.create_session();

    if let Ok(_) = create_session_result {
        println!("Authenticated to BSKY");

        let create_response = client.create_record::<bsky_client::types::Post, serde_json::Value>(
            String::from("app.bsky.feed.post"),
            Post {
                text: String::from("Hello, World (this is a test don't panic)"),
                created_at: String::from("2023-07-03T03:19:38.877Z"),
            },
        );

        match create_response {
            Ok(response) => {
                println!("Sent!");
                println!("{:?}", response);
            }
            Err(error) => {
                println!("HttpError!");
                println!("{:?}", error);
            }
        }
    }
}
