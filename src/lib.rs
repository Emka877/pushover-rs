#[cfg(test)]
mod tests;
mod pushover;

use pushover::{constants::PUSHOVER_API_ENDPOINT};
pub use pushover::data::MessageBuilder;
pub use pushover::data::PushoverSound;
pub use pushover::data::Message;
pub use pushover::data::PushoverResponse;

pub async fn send_pushover_request(message: Message) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(PUSHOVER_API_ENDPOINT)
        .json(&message)
        .send()
        .await?;
    PushoverResponse::try_from_reqwest_response(response).await
}
