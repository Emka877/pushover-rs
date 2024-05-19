/**
    Basic example for the pushover-rs usage.

    Try this example by running: `cargo run --example usage-example`
**/

extern crate pushover_rs;
use pushover_rs::{Message, MessageBuilder, PushoverResponse, PushoverSound, send_pushover_request};
use std::time::{SystemTime, UNIX_EPOCH};

mod common;
use common::*;

async fn something_happened_send_notification() -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    // Reads the credentials from a file, feel free to use anything else to store your own credentials.
    let credentials: ExampleCredentials = read_credentials();
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();
    let message: Message = MessageBuilder::new(&credentials.user, &credentials.token, "Example message")
        .set_title("Example push notification sent through Pushover API")
        .set_url("https://pushover.net/", Some("Pushover"))
        .set_priority(1)
        .set_ttl(60) // 60 seconds. Note that setting a TTL will overwrite set_priority to value 2! See: https://pushover.net/api#ttl
        .set_sound(PushoverSound::ALIEN)
        .set_timestamp(now)
        .build();
    
    send_pushover_request(message).await
}

#[tokio::main]
pub async fn main() {
    let response: Result<PushoverResponse, Box<dyn std::error::Error>> = something_happened_send_notification().await;

    // Handle errors or not, it's up to you
    // If you decide to handle them, know that there are 2 possible levels of errors:
    // 1) HTTP protocol error, when, for example, the endpoint address is incorrect, or the API is down
    // 2) Request error: When your request doesn't contain all the required info, namely, "user key", "app token" and "message"
    // This is why error checking looks so extensive
    
    println!("The request was sent, let's check if it was correct...");
    
    // HTTP protocol error level
    if response.is_ok() {
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
        } else {
            println!("Ok, your push notification should arrive soon.");
        }
    }
}
