use teloxide::prelude2::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CatMateMessage {
    chat_id: i64,
    sent: bool,
}

impl CatMateMessage {
    pub fn new(chat_id: i64) -> Self {
        Self {
            sent: false,
            chat_id,
        }
    }

    pub fn get_next(&self) -> Self {
        Self::new(self.chat_id)
    }

    pub fn not_sent_yet(&self) -> bool {
        !self.sent
    }

    pub fn mark_as_sent(&mut self) {
        self.sent = true;
    }

    pub async fn send(&self, bot: &'static AutoSend<Bot>) {
        bot.send_message(self.chat_id, self.get_text())
            .await
            .expect(&format!("Could not send message to {}", self.chat_id.to_string()));
    }

    fn get_text(&self) -> String {
        "TODO message text body here".to_string()
    }
}
