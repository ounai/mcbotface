use std::{
    env,
    sync::{Arc, Mutex},
};

use teloxide::prelude2::*;
use lazy_static::lazy_static;

mod scheduler;
mod commands;
mod catmate;
mod tj;

use catmate::CatMateMessage;

#[tokio::main]
async fn main() {
    let chat_id = env::var("CHAT_ID")
        .expect("CHAT_ID is not set")
        .parse::<i64>()
        .expect("CHAT_ID is not a number");

    println!("Initializing Telegram bot...");

    lazy_static! {
        static ref BOT: AutoSend<Bot> = Bot::from_env().auto_send();
    }

    // TODO load from file
    let catmate_message = Arc::new(Mutex::new(CatMateMessage::new(chat_id)));

    scheduler::init(&*BOT, catmate_message);
    commands::listen(&*BOT).await
}
