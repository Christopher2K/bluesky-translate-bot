mod bsky_client;

fn main() {
    let mut client = bsky_client::client::Client::new(
        String::from("llcoolchris.dev"),
        String::from(""),
    );
    let create_session_result = client.create_session();

    if let Ok(_) = create_session_result {
        println!("Authenticated to BSKY");
        println!("{:?}", &client);
    }
}
