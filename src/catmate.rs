use teloxide::prelude2::*;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CatMateMessage {
    chat_id: i64,
    sent: bool,
    created_at: i64,
}

impl CatMateMessage {
    pub fn new(chat_id: i64) -> Self {
        Self {
            sent: false,
            chat_id,
            created_at: Utc::now().timestamp(),
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

    pub fn get_datetime_string(&self) -> String {
        let datetime = NaiveDateTime::from_timestamp(self.created_at, 0);

        format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
    }

    fn get_text(&self) -> String {
        format!("{} CatMate(tm)\n{:?}", self.get_datetime_string(), self)
    }
}
