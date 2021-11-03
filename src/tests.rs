use std::{io::BufReader, path::PathBuf};

use crate::pushover::data::*;
use crate::{send_pushover_request, send_pushover_request_with_attachment};
use serde::Deserialize;

/* Test setup */

#[derive(Debug, Clone, Deserialize)]
struct TestData {
    #[serde(alias = "token")]
    app_token: String,

    #[serde(alias = "user")]
    user_key: String,
}

fn read_test_data() -> Result<TestData, Box<dyn std::error::Error>> {
    let file_path: PathBuf = PathBuf::from("./testdata/credentials.json");
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

/* Actual tests */
#[test]
fn test_testdata_readability() {
    let testdata: Result<TestData, Box<dyn std::error::Error>> = read_test_data();
    assert_eq!(testdata.is_ok(), true);
}

#[test]
fn test_message_builder() {
    /* Minimal message */
    let mb1: MessageBuilder = MessageBuilder::new("abc", "def", "test message");
    let m1: Message = mb1.build();

    assert_eq!(m1.app_token, "def".to_owned());
    assert_eq!(m1.user_key, "abc".to_owned());
    assert_eq!(m1.message, "test message".to_owned());

    /* Full message */
    let mfull: Message = MessageBuilder::new("abc", "def", "test")
        .add_title("Title")
        .add_url("https://www.google.be/", Some("Google"))
        .modify_message("test message")
        .set_priority(100) // "Error": Out of the [-2, 2] boundary -> Should be reset to 0
        .set_sound(PushoverSound::CASHREGISTER)
        .set_timestamp(1635861224)
        .add_device("home desktop")
        .build();

    assert_eq!(mfull.user_key, "abc".to_owned());
    assert_eq!(mfull.app_token, "def".to_owned());
    assert_eq!(mfull.message, "test message".to_owned());
    assert_eq!(mfull.title, Some("Title".to_owned()));
    assert_eq!(mfull.url, Some("https://www.google.be/".to_owned()));
    assert_eq!(mfull.url_title, Some("Google".to_owned()));
    assert_eq!(mfull.priority, Some(0));
    assert_eq!(mfull.sound, Some("cashregister".to_owned()));
    assert_eq!(mfull.timestamp, Some(1635861224));
    assert_eq!(mfull.devices, Some(vec!["home desktop".to_owned()]));
}

#[tokio::test]
async fn test_send_request_minimal_message() {
    if let Ok(credentials) = read_test_data() {
        let message: Message = MessageBuilder::new(
            credentials.user_key.as_str(),
            credentials.app_token.as_str(),
            "Test from pushover-rs.",
        ).build();
        let response = send_pushover_request(message).await;
        assert_eq!(response.is_ok(), true);
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
            "Test from pushover-rs.",
        )
        .set_attachment(attachment_path)
        .build()
        .unwrap();
        
        let response = send_pushover_request_with_attachment(message);
        
        assert_eq!(response.is_ok(), true);
    }
}
