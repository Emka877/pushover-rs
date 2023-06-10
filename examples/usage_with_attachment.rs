/*
 * TODO: Create the example
 * 
 * Basic usage example with an attachment.
 * 
 * Adding an attachment to a pushover request renders said request blocking.
 */

extern crate pushover_rs;
mod common;
use common::*;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use pushover_rs::{AttachmentMessageBuilder, send_pushover_request_with_attachment, PushoverResponse, PushoverSound};

pub fn send_pushover_message_with_attachment() -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let credentials: ExampleCredentials = read_credentials();
    let duration_since_epoch: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();
    let message = AttachmentMessageBuilder::new(&credentials.user, &credentials.token, "Some message")
        .set_title("Example with attachment")
        .set_attachment("examples/data/attachment.jpg".into())
        .set_sound(PushoverSound::TUGBOAT)
        .set_timestamp(now)
        .build()
        .unwrap();
    send_pushover_request_with_attachment(message)
}

// Notice how main is not async, as adding an attachment renders the query blocking.
pub fn main() {
    let results: Result<PushoverResponse, Box<dyn std::error::Error>> =
        send_pushover_message_with_attachment();
    
    // Check for HTTP protocol level errors
    if results.is_ok() {
        let pushover_response: PushoverResponse = results.ok().unwrap();

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
