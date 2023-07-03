mod bsky_client;

use bsky_client::client::Client;
use bsky_client::types::Post;

fn main() {
    let mut client = Client::new(
        String::from("llcoolchris.dev"),
        String::from(""),
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
