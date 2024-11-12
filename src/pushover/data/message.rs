use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
/**
A message to be used in conjuction with the send_pushover_request function.

Note: It is preferred to create a Message through the MessageBuilder.
 **/
pub struct Message {
    /* Required */
    /// (Required) Your app API token, see https://pushover.net/apps/[your application ID]
    #[serde(rename = "token")]
    pub app_token: String,
    /// (Required) Your User key, see your dashboard (https://pushover.net/ top-right)
    #[serde(rename = "user")]
    pub user_key: String,
    /// (Required) Your message
    pub message: String,

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
    pub priority: Option<i8>,
    /// The name of one of the sounds supported by device clients to override the user's default sound choice. (See sound list: https://pushover.net/api#sounds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// A Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>, // Year 2038 proof :p
    /// A list of device names to send the push notifications to, if you want to limit the notification to certain devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    /// A TTL (Time to Live) in seconds, after which the message will be automatically deleted from the recipient's inbox.
    /// Setting *ttl* to None prevents this auto removal. Setting TTL to 0 will raise an error (ttl must be > 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            app_token: "".into(),
            user_key: "".into(),
            message: "No message set.".into(),
            title: None,
            url: None,
            url_title: None,
            priority: None,
            sound: None,
            timestamp: None,
            devices: None,
            ttl: None,
        }
    }
}
