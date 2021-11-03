use std::io::Error;
use std::io::ErrorKind;

use super::PushoverSound;
use super::AttachmentMessage;

pub struct AttachmentMessageBuilder {
    build: AttachmentMessage,
}

impl AttachmentMessageBuilder {
    /// Creates a new MessageBuilder instance with the required minimal informations (User key, App token & Message)
    pub fn new(user_key: &str, application_token: &str, message: &str) -> Self {
        let mut build = AttachmentMessage::default();
        
        build.user_key = user_key.to_owned();
        build.app_token = application_token.to_owned();
        build.message = message.to_owned();

        AttachmentMessageBuilder {
            build,
        }
    }

    /// Modifies the existing message.
    pub fn modify_message(mut self, message: &str) -> AttachmentMessageBuilder {
        if message.trim().len() == 0 {
            return self;
        }

        self.build.message = message.to_owned();
        self
    }

    /// Adds a title to your message
    pub fn add_title(mut self, title: &str) -> AttachmentMessageBuilder {
        if title.trim().len() == 0 {
            self.build.title = None;
        }

        self.build.title = Some(title.to_owned());
        self
    }

    /// Removes the title. The title will be defaulted to your application name.
    pub fn remove_title(mut self) -> AttachmentMessageBuilder {
        self.build.title = None;
        self
    }

    /// Adds an url (and optionally, an url title) to send along with your message.
    /// 
    /// If set, the URL title will be shown, otherwise the URL will be shown.
    pub fn add_url(mut self, url: &str, url_title: Option<&str>) -> AttachmentMessageBuilder {
        if url.trim().len() == 0 {
            self.build.url = None;
            self.build.url_title = None;
            return self;
        }

        self.build.url = Some(url.to_owned());
        if url_title.is_some() {
            self.build.url_title = Some(url_title.unwrap().to_owned());
        }
        self
    }

    /// Removes both the url and url title from your message
    pub fn remove_url(mut self) -> AttachmentMessageBuilder {
        self.build.url = None;
        self.build.url_title = None;
        self
    }

    /// Send as -2 to generate no notification/alert, -1 to always send as a quiet notification, 1 to display as high-priority and bypass the user's quiet hours, or 2 to also require confirmation from the user.
    pub fn set_priority(mut self, priority: i8) -> AttachmentMessageBuilder {
        if priority < -2 || priority > 2 {
            self.build.priority = Some("0".into());
            return self;
        }

        self.build.priority = Some(priority.to_string());
        self
    }

    /// Resets the priority to default (0, normal)
    pub fn remove_priority(mut self) -> AttachmentMessageBuilder {
        self.build.priority = Some("0".into());
        self
    }

    /// Sets the sound to be used to notify the user.
    /// 
    /// See this list of available sounds: https://pushover.net/api#sounds
    pub fn set_sound(mut self, sound: PushoverSound) -> AttachmentMessageBuilder {
        self.build.sound = Some(sound.to_string());
        self
    }

    /// Removes the custom sound and reverts to the default sound.
    pub fn remove_sound(mut self) -> AttachmentMessageBuilder {
        self.build.sound = None;
        self
    }

    /// Sets an Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    pub fn set_timestamp(mut self, unix_timestamp: u64) -> AttachmentMessageBuilder {
        self.build.timestamp = Some(unix_timestamp.to_string());
        self
    }

    /// Resets the custom unix timestamp
    pub fn remove_timestamp(mut self) -> AttachmentMessageBuilder {
        self.build.timestamp = None;
        self
    }

    /// Add a device name to send the notification to.
    pub fn add_device(mut self, device_name: &str) -> AttachmentMessageBuilder {
        type Devices = Vec<String>;

        if device_name.trim().len() == 0 {
            return self;
        }

        if self.build.devices.is_none() {
            self.build.devices = Some(vec!());
        }

        let mut replacement_list: Devices = self.build.devices.clone().unwrap();
        replacement_list.push(device_name.to_owned());
        self.build.devices = Some(replacement_list);

        self
    }

    pub fn clear_devices_list(mut self) -> AttachmentMessageBuilder {
        self.build.devices = None;
        self
    }

    /// Add attachment to the message.
    /// 
    /// Attachments cannot be larger than 2.5MB.
    /// 
    /// Adding an attachment will make the API call blocking.
    pub fn set_attachment(mut self, attachment_path: String) -> AttachmentMessageBuilder {
        if attachment_path.trim().len() == 0 {
            return self;
        }
        self.build.attachment = attachment_path;
        self
    }

    /// Transforms the MessageBuilder into a useable Message
    pub fn build(self) -> Result<AttachmentMessage, Box<dyn std::error::Error>> {
        if self.build.app_token.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Application token is empty")));
        }

        if self.build.user_key.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "User key is empty")));
        }

        if self.build.message.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Message is empty")));
        }

        if self.build.attachment.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Attachment is empty")));
        }

        Ok(self.build.clone())
    }
}