/**
    Basic example for the pushover-rs usage.

    Try this example by running: `cargo run --example usage-example`
**/
extern crate pushover_rs;
use std::fs::File;

use pushover_rs::{Message, MessageBuilder, PushoverResponse, PushoverSound, send_pushover_request};
use serde::Deserialize;
use serde_json::from_reader;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct ExampleCredentials {
    user: String,
    token: String,
}

fn read_credentials() -> ExampleCredentials {
    let path: String = "./data/credentials.ron".into();
    let file: File = File::open(&path).expect(&format!("Cannot find file at location: {}", path));
    match from_reader(file) {
        Ok(creds) => creds,
        Err(err) => {
            eprintln!("Error reading credentials file: {}", err);
            std::process::exit(1);
        },
    }
}

async fn something_happened_send_notification() -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    // Reads the credentials from a file, feel free to use anything else to store your own credentials.
    let credentials: ExampleCredentials = read_credentials();
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();
    let message: Message = MessageBuilder::new(&credentials.user, &credentials.token, "Example message")
        .add_title("Example push notification sent through Pushover API")
        .add_url("https://pushover.net/", Some("Pushover"))
        .set_priority(1)
        .set_sound(PushoverSound::ALIEN)
        .set_timestamp(now)
        .build();
    
    send_pushover_request(message).await
}

#[tokio::main]
pub async fn main() {
    let response = something_happened_send_notification().await;

    // Handle errors or not, it's up to you
    // If you decide to handle them, know that there are 2 possible levels of errors:
    // 1) HTTP protocol error, when, for example, the endpoint address is incorrect, or the API is down
    // 2) Request error: When your request doesn't contain all the required info, namely, "user key", "app token" and "message"
    // This is why error checking looks so extensive
    
    // HTTP protocol error level
    if response.is_ok() {
        eprintln!("The request was sent, let's check if it was correct...");
        let pushover_response: PushoverResponse = response.ok().unwrap();

        if pushover_response.status != 1 {
            // Request level error(s)
            eprintln!("The request was incorrect.");

            if pushover_response.errors.is_some() {
                let errors = pushover_response.errors
                    .unwrap()
                    .into_iter()
                    .map(|x| format!("{} ", x))
                    .collect::<String>();
                eprintln!("The request was incorrect: {}", errors);
            } else {
                eprintln!("The request was incorrect.", )
            }
        }
    }
}
