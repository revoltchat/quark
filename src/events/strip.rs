// this was just an experiment, ignore this file for now

trait StripExcessData: Sized {
    fn strip_self(mut self) -> Self {
        self.strip();
        self
    }

    fn strip(&mut self);
}

impl StripExcessData for Server {
    fn strip(&mut self) {
        self.description = None;
        self.categories = None;
        self.system_messages = None;
        self.icon = None;
        self.banner = None;
        self.flags = None;
    }
}

impl StripExcessData for Member {
    fn strip(&mut self) {
        self.nickname = None;
        self.avatar = None;
    }
}

impl StripExcessData for User {
    fn strip(&mut self) {
        self.avatar = None;
        self.relations = None;
        self.badges = None;
        self.status
    }
}

impl StripExcessData for Channel {
    fn strip(&mut self) {
        match self {
            Self::DirectMessage {
                last_message_id, ..
            } => {
                *last_message_id = None;
            }
            Self::Group {
                description,
                icon,
                last_message_id,
                permissions,
                ..
            } => {
                *description = None;
                *icon = None;
                *last_message_id = None;
                *permissions = None;
            }
            Self::TextChannel {
                description,
                icon,
                last_message_id,
                default_permissions,
                ..
            } => {
                *description = None;
                *icon = None;
                *last_message_id = None;
                *default_permissions = None;
            }
            Self::VoiceChannel {
                description,
                icon,
                default_permissions,
                ..
            } => {
                *description = None;
                *icon = None;
                *default_permissions = None;
            }
            _ => {}
        }
    }
}
