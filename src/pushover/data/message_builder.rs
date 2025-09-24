use super::{Message, PushoverSound};

// TODO: Fix DRY principle with attachment_message_builder.rs

/**
Helps build a correct Pushover request.
 */
#[derive(Debug)]
pub struct MessageBuilder {
    build: Message,
}

#[allow(dead_code)]
impl MessageBuilder {
    /// Creates a new MessageBuilder instance with the required minimal informations (User key, App token & Message)
    pub fn new(user_key: &str, application_token: &str, message: &str) -> Self {
        let mut build = Message::default();
        
        build.user_key = user_key.to_owned();
        build.app_token = application_token.to_owned();
        build.message = message.to_owned();

        MessageBuilder {
            build,
        }
    }

    /// Modifies the existing message.
    pub fn modify_message(mut self, message: &str) -> MessageBuilder {
        if message.trim().len() == 0 {
            return self;
        }

        self.build.message = message.to_owned();
        self
    }

    /// Sets a title to your message
    pub fn set_title(mut self, title: &str) -> MessageBuilder {
        if title.trim().len() == 0 {
            self.build.title = None;
        }

        self.build.title = Some(title.to_owned());
        self
    }

    /// Adds a title to your message
    #[deprecated(since="0.3.12", note="Please use set_title instead.")]
    pub fn add_title(mut self, title: &str) -> MessageBuilder {
        if title.trim().len() == 0 {
            self.build.title = None;
        }

        self.build.title = Some(title.to_owned());
        self
    }

    /// Removes the title. The title will be defaulted to your application name.
    pub fn remove_title(mut self) -> MessageBuilder {
        self.build.title = None;
        self
    }

    /// Sets an url (and optionally, an url title) to send along with your message.
    ///
    /// If set, the URL title will be shown, otherwise the URL will be shown.
    pub fn set_url(mut self, url: &str, url_title: Option<&str>) -> MessageBuilder {
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

    /// Adds an url (and optionally, an url title) to send along with your message.
    /// 
    /// If set, the URL title will be shown, otherwise the URL will be shown.
    #[deprecated(since="0.3.12", note="Please use set_url instead.")]
    pub fn add_url(mut self, url: &str, url_title: Option<&str>) -> MessageBuilder {
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
    pub fn remove_url(mut self) -> MessageBuilder {
        self.build.url = None;
        self.build.url_title = None;
        self
    }

    /// Send as -2 to generate no notification/alert, -1 to always send as a quiet notification, 1 to display as high-priority and bypass the user's quiet hours, or 2 to also require confirmation from the user.
    pub fn set_priority(mut self, priority: i8) -> MessageBuilder {
        if priority < -2 || priority > 2 {
            self.build.priority = Some(0);
            return self;
        }

        self.build.priority = Some(priority);
        self
    }

    /// Resets the priority to default (0, normal)
    pub fn remove_priority(mut self) -> MessageBuilder {
        self.build.priority = Some(0);
        self.build.retry = None;
        self.build.expire = None;
        self
    }

    /// When the priority is set to 2, sets the amount of seconds between each retries. Must be at least 30 seconds.
    pub fn set_retry(mut self, retry_secs: i32) -> MessageBuilder {
        if self.build.priority != Some(2) {
            // Retry only makes sense if priority is 2
            return self;
        }

        if retry_secs < 30 {
            self.build.retry = Some(30);
            return self;
        }

        self.build.retry = Some(retry_secs);
        self
    }

    /// When the priority is set to 2, sets the amount of seconds before the notification is expired. The maximum value is 10800 (3 hours). Must be between 60 and 10800.
    pub fn set_expire(mut self, expire_secs: i32) -> MessageBuilder {
        if self.build.priority != Some(2) {
            // Expire only makes sense if priority is 2
            return self;
        }

        if expire_secs < 60 {
            self.build.expire = Some(60);
            return self;
        }
        else if expire_secs > 10800 {
            self.build.expire = Some(10800);
            return self;
        }

        self.build.expire = Some(expire_secs);
        self
    }

    /// Sets the sound to be used to notify the user.
    /// 
    /// See this list of available sounds: https://pushover.net/api#sounds
    pub fn set_sound(mut self, sound: PushoverSound) -> MessageBuilder {
        self.build.sound = Some(sound.to_string());
        self
    }

    /// Removes the custom sound and reverts to the default sound.
    pub fn remove_sound(mut self) -> MessageBuilder {
        self.build.sound = None;
        self
    }

    /// Sets an Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    pub fn set_timestamp(mut self, unix_timestamp: u64) -> MessageBuilder {
        self.build.timestamp = Some(unix_timestamp);
        self
    }

    /// Resets the custom unix timestamp
    pub fn remove_timestamp(mut self) -> MessageBuilder {
        self.build.timestamp = None;
        self
    }

    /// Add a device name to send the notification to.
    ///
    /// Overrides the current device if a new device name is set.
    pub fn set_device(mut self, device_name: &str) -> MessageBuilder {
        self.build.device = Some(device_name.to_string());
        self
    }

    /// Clears the device if set.
    pub fn remove_device(mut self) -> MessageBuilder {
        self.build.device = None;
        self
    }

    /// Set the TTL (Time to Live), in seconds
    pub fn set_ttl(mut self, ttl_secs: u32) -> MessageBuilder {
        if ttl_secs <= 0 {
            self.build.ttl = None;
        }
        else {
            self.build.ttl = Some(ttl_secs);
        }
        self
    }

    /// Transforms the MessageBuilder into a usable Message
    pub fn build(mut self) -> Message {
        if self.build.priority == Some(2) {
            if self.build.retry.is_none() {
                self.build.retry = Some(30);
            }
            if self.build.expire.is_none() {
                self.build.expire = Some(10800);
            }
        }
        self.build
    }
}
