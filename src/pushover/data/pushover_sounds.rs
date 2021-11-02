pub enum PushoverSound {
    PUSHOVER,
    BIKE,
    BUGLE,
    CASHREGISTER,
    CLASSICAL,
    COSMIC,
    FALLING,
    GAMELAN,
    INCOMING,
    INTERMISSION,
    MAGIC,
    MECHANICAL,
    PIANOBAR,
    SIREN,
    SPACEALARM,
    TUGBOAT,
    ALIEN,
    CLIMB,
    PERSISTENT,
    ECHO,
    UPDOWN,
    VIBRATE,
    NONE,
}

// Enables PushoverSound::[SOUNDNAME].to_string();
impl std::fmt::Display for PushoverSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = match *self {
            PushoverSound::PUSHOVER => "pushover",
            PushoverSound::BIKE => "bike",
            PushoverSound::BUGLE => "bugle",
            PushoverSound::CASHREGISTER => "cashregister",
            PushoverSound::CLASSICAL => "classical",
            PushoverSound::COSMIC => "cosmic",
            PushoverSound::FALLING => "falling",
            PushoverSound::GAMELAN => "gamelan",
            PushoverSound::INCOMING => "incoming",
            PushoverSound::INTERMISSION => "intermission",
            PushoverSound::MAGIC => "magic",
            PushoverSound::MECHANICAL => "mechanical",
            PushoverSound::PIANOBAR => "pianobar",
            PushoverSound::SIREN => "siren",
            PushoverSound::SPACEALARM => "spacealarm",
            PushoverSound::TUGBOAT => "tugboat",
            PushoverSound::ALIEN => "alien",
            PushoverSound::CLIMB => "climb",
            PushoverSound::PERSISTENT => "persistent",
            PushoverSound::ECHO => "echo",
            PushoverSound::UPDOWN => "updown",
            PushoverSound::VIBRATE => "vibrate",
            PushoverSound::NONE => "none",
        };

        write!(f, "{}", name)
    }
}
