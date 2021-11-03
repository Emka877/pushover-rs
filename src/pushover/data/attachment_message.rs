use reqwest::blocking::multipart::Form;

#[derive(Debug, Clone)]
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
    pub title: Option<String>,
    /// A supplementary URL to show with your message
    pub url: Option<String>,
    /// A title for your supplementary URL, otherwise just the URL is shown
    pub url_title: Option<String>,
    /// Send as -2 to generate no notification/alert, -1 to always send as a quiet notification, 1 to display as high-priority and bypass the user's quiet hours, or 2 to also require confirmation from the user
    pub priority: Option<String>,
    /// The name of one of the sounds supported by device clients to override the user's default sound choice. (See sound list: https://pushover.net/api#sounds)
    pub sound: Option<String>,
    /// A Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    pub timestamp: Option<String>, // Year 2038 proof :p
    /// A list of device names to send the push notifications to, if you want to limit the notification to certain devices.
    pub devices: Option<Vec<String>>,
}

impl AttachmentMessage {
    pub fn into_form(self) -> Result<Form, std::io::Error> {
        Form::new()
            .text("token", self.app_token.clone())
            .text("user", self.user_key.clone())
            .text("message", self.message.clone())
            .text("title", self.title.clone().unwrap_or("".into()))
            .text("url", self.url.clone().unwrap_or("".into()))
            .text("url_title", self.url_title.clone().unwrap_or("".into()))
            .text("priority", self.priority.unwrap_or("".into()))
            .text("sound", self.sound.clone().unwrap_or("".into()))
            .text("timestamp", self.timestamp.unwrap_or("".into()))
            .text("device", self.devices.clone().unwrap_or(vec!()).join(","))
            .file("attachment", self.attachment.clone())
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
            devices: None,
        }
    }
}