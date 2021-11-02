/**
    Basic example for the pushover-rs usage.

    Try this example by running: `cargo run --example usage-example`
**/
extern crate pushover_rs;
use pushover_rs::{Message, MessageBuilder, PushoverResponse, PushoverSound, send_pushover_request};

async fn something_happened_send_notification() -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let message: Message = MessageBuilder::new("your user key", "your app token", "Your message")
        .add_title("Some title")
        .add_url("https://www.google.com/", Some("Google"))
        .set_priority(1)
        .set_sound(PushoverSound::ALIEN)
        .set_timestamp(1635871928)
        .build();
    send_pushover_request(message).await
}

#[tokio::main]
pub async fn main() {
    let response = something_happened_send_notification().await;

    if response.is_ok() {
        println!("The request was sent, let's check if it was correct...");
        let pushover_response: PushoverResponse = response.ok().unwrap();

        if pushover_response.status == 1 {
            println!("The request was correct.");
        } else {
            println!("The request was incorrect.");

            if pushover_response.errors.is_some() {
                let errors = pushover_response.errors
                    .unwrap()
                    .into_iter()
                    .map(|x| format!("{} ", x))
                    .collect::<String>();
                println!("The request was incorrect: {}", errors);
            } else {
                println!("The request was incorrect.", )
            }
        }
    }
}
