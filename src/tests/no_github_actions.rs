use crate::tests::setup::{TestData, read_test_data};
use crate::{send_pushover_request, send_pushover_request_with_attachment};
use crate::{
    pushover::data::{Message, MessageBuilder, AttachmentMessage, AttachmentMessageBuilder, PushoverResponse}
};

#[test]
pub fn test_testdata_readability() {
    let testdata: Result<TestData, Box<dyn std::error::Error>> = read_test_data();
    assert_eq!(testdata.is_ok(), true);
}

#[tokio::test]
async fn test_send_request_minimal_message() {
    if let Ok(credentials) = read_test_data() {
        let message: Message = MessageBuilder::new(
            credentials.user_key.as_str(),
            credentials.app_token.as_str(),
            "Test from pushover-rs.",
        )
        .build();
        let response = send_pushover_request(message).await;
        assert_eq!(response.is_ok(), true);
    } else {
        panic!("Could not read test data.");
    }
}

#[tokio::test]
async fn test_send_request_min_message_with_ttl() {
    if let Ok(credentials) = read_test_data() {
        let ttl: u32 = 5;
        let message: Message = MessageBuilder::new(
            credentials.user_key.as_str(),
            credentials.app_token.as_str(),
            format!("Test from pushover-rs with {ttl} seconds TTL").as_str(),
        )
        .set_ttl(ttl)
        .build();
        let response = send_pushover_request(message).await;
        assert_eq!(response.is_ok(), true);
    } else {
        panic!("Could not read test data.");
    }
}

#[tokio::test]
async fn test_send_bad_request() {
    // Message should not be used "manually", messagebuilder should be used.
    // Default message doesn't contain the minimal info for a request.
    let message = Message {
        ..Default::default()
    };
    let response = send_pushover_request(message).await;

    assert_eq!(response.is_ok(), true); // Bad request doesn't mean the request didn't go through

    if response.is_ok() {
        // !Shadowing previous var
        let response: PushoverResponse = response.ok().unwrap();
        assert_eq!(response.status, 0);
    }
}

#[test]
fn test_send_with_good_attachment() {
    if let Ok(credentials) = read_test_data() {
        let attachment_path: String = "./testdata/attachment_test.jpg".to_owned();
        let message: AttachmentMessage = AttachmentMessageBuilder::new(
            credentials.user_key.as_str(),
            credentials.app_token.as_str(),
            "Test from pushover-rs, with attachment and ttl",
        )
        .set_attachment(attachment_path)
        .set_ttl(10)
        .build()
        .unwrap();

        let response = send_pushover_request_with_attachment(message);

        assert_eq!(response.is_ok(), true);
    } else {
        panic!("Could not read test data.");
    }
}
