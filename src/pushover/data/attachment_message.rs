use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::io;

#[derive(Debug, Clone, Serialize)]
/**
 A message containing an attachment, to be used in conjuction with the send_pushover_request_with_attachment function.

 Note: It is preferred to create a Message through the AttachmentMessageBuilder.
 **/
pub struct AttachmentMessage {
    /* Required */
    /// (Required) Your app API token, see https://pushover.net/apps/[your application ID]
    pub app_token: String,
    /// (Required) Your User key, see your dashboard (https://pushover.net/ top-right)
    pub user_key: String,
    /// (Required) Your message
    pub message: String,
    /// (Required) An attachment file path to send with the message. (Max size: 2,5MB)
    pub attachment: String,

    /* Optional */
    /// The title of the message, otherwise your app's name will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A supplementary URL to show with your message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A title for your supplementary URL, otherwise just the URL is shown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_title: Option<String>,
    /// Send as -2 to generate no notification/alert, -1 to always send as a quiet notification, 1 to display as high-priority and bypass the user's quiet hours, or 2 to also require confirmation from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// The name of one of the sounds supported by device clients to override the user's default sound choice. (See sound list: https://pushover.net/api#sounds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// A Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>, // Year 2038 proof :p
    /// A device name to send the push notifications to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// A TTL (Time to Live) in seconds, after which the message will be automatically deleted from the recipient's inbox.
    /// Setting *ttl* to None prevents this auto removal. Setting TTL to 0 will raise an error (ttl must be > 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

impl AttachmentMessage {
    pub fn into_form(self) -> Result<reqwest::blocking::multipart::Form, std::io::Error> {
        let mut form: reqwest::blocking::multipart::Form = reqwest::blocking::multipart::Form::new()
            .text("token", self.app_token.clone())
            .text("user", self.user_key.clone())
            .text("message", self.message.clone())
            .text("title", self.title.clone().unwrap_or(String::from("")))
            .text("url", self.url.clone().unwrap_or(String::from("")))
            .text("url_title", self.url_title.clone().unwrap_or(String::from("")))
            .text("priority", self.priority.unwrap_or(String::from("")))
            .text("sound", self.sound.clone().unwrap_or(String::from("")))
            .text("timestamp", self.timestamp.unwrap_or(String::from("")))
            .text("device", self.device.clone().unwrap_or(String::from("")));
        // TTL became required if it has a value, 0 doesn't work anymore.
        if self.ttl.is_some() {
            form = form.text("ttl", self.ttl.unwrap_or(999).to_string());
        }
        let attachment_part = reqwest::blocking::multipart::Part::bytes(std::fs::read(&self.attachment)?)
            .file_name(self.attachment.clone());
        Ok(form.part("attachment", attachment_part))
    }

    pub async fn into_form_async(self) -> Result<Form, io::Error> {
        let mut form = Form::new()
            .text("token", self.app_token.clone())
            .text("user", self.user_key.clone())
            .text("message", self.message.clone())
            .text("title", self.title.clone().unwrap_or(String::from("")))
            .text("url", self.url.clone().unwrap_or(String::from("")))
            .text("url_title", self.url_title.clone().unwrap_or(String::from("")))
            .text("priority", self.priority.unwrap_or(String::from("")))
            .text("sound", self.sound.clone().unwrap_or(String::from("")))
            .text("timestamp", self.timestamp.unwrap_or(String::from("")))
            .text("device", self.device.clone().unwrap_or(String::from("")));
        if self.ttl.is_some() {
            form = form.text("ttl", self.ttl.unwrap_or(999).to_string());
        }
        let file_bytes = tokio::fs::read(&self.attachment).await?;
        let part = Part::bytes(file_bytes)
            .file_name(self.attachment.clone());
        Ok(form.part("attachment", part))
    }
}

impl Default for AttachmentMessage {
    fn default() -> Self {
        Self {
            app_token: "".into(),
            user_key: "".into(),
            message: "".into(),
            attachment: "".into(),
            title: None,
            url: None,
            url_title: None,
            priority: None,
            sound: None,
            timestamp: None,
            device: None,
            ttl: None,
        }
    }
}
