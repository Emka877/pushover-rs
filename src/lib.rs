#[cfg(test)]
mod tests;
mod pushover;

use pushover::{constants::PUSHOVER_API_ENDPOINT};
pub use pushover::data::{MessageBuilder, AttachmentMessageBuilder};
pub use pushover::data::PushoverSound;
pub use pushover::data::Message;
pub use pushover::data::AttachmentMessage;
pub use pushover::data::PushoverResponse;

/// Send a push notification without attachment (non blocking)
pub async fn send_pushover_request(message: Message) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(PUSHOVER_API_ENDPOINT)
        .json(&message)
        .send()
        .await?;
    return PushoverResponse::try_from_reqwest_response(response).await;
}

/// Send a push notification with attachment (! blocking)
pub fn send_pushover_request_with_attachment(message: AttachmentMessage) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = message.into_form()?;

    let response = client
        .post(PUSHOVER_API_ENDPOINT)
        .multipart(form)
        .send();
    
    return PushoverResponse::try_from_blocking_reqwest_response(response.unwrap());
}
