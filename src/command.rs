use std::fmt;

use telegram_bot::{
    User,
    Message,
};

fn get_sender(message: &Message) -> String {
    match &message.from {
        // @username
        User { username: Some(username), .. } =>
            ["@".to_string(), username.to_string()].concat(),

        // firstname lastname
        User { first_name, last_name: Some(last_name), .. } =>
            [first_name.to_string(), last_name.to_string()].join(" "),

        // firstname
        User { first_name, .. } =>
            first_name.to_string(),
    }
}

pub struct Command {
    sender: String,
    text: String,
    message: Message,
}

impl Command {
    pub fn new(message: Message, text: String) -> Self {
        Self {
            sender: get_sender(&message),
            text,
            message,
        }
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.text.starts_with(s)
    }

    pub fn get_message(&self) -> &Message {
        &self.message
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sender, self.text)
    }
}
