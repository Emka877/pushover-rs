mod pushover;
#[cfg(test)]
mod tests;

use pushover::{constants::PUSHOVER_API_ENDPOINT, data::{Message, PushoverResponse}};

pub async fn send_request(message: Message) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(PUSHOVER_API_ENDPOINT)
        .json(&message)
        .send()
        .await?;
    PushoverResponse::try_from_reqwest_response(response).await
}
