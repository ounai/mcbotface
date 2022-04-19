use std::error::Error;

use teloxide::{
    prelude2::*,
    utils::command::BotCommand,
    types::User,
};

use chrono::prelude::*;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "send a hello message")]
    Hello,

    #[command(description = "count days to a date")]
    Tj,
}

trait GetSender {
    fn get_sender(&self) -> String;
}

impl GetSender for Message {
    fn get_sender(&self) -> String {
        match &self.from().unwrap() {
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
}

fn get_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn get_days_until(date: &str) -> String {
    let now = Utc::now();

    let then = DateTime::parse_from_rfc3339(&[date, "T00:00:00+00:00"].concat())
        .expect("Invalid date string");

    let duration = then.signed_duration_since(now).to_std().unwrap();

    let days = (duration.as_secs() / 60 / 60 / 24) + 1;

    format!("{} days until {}", days, date)
}

async fn on_command(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Hello =>
            bot.send_message(message.chat.id, get_hello(&message.get_sender()))
                .reply_to_message_id(message.id)
                .await?,

        Command::Tj =>
            bot.send_message(message.chat.id, get_days_until("2022-07-01"))
                .reply_to_message_id(message.id)
                .await?,
    };

    Ok(())
}

pub async fn listen(bot: &'static AutoSend<Bot>) {
    println!("Initializing command listeners...");

    let listener = teloxide::repls2::commands_repl(bot.clone(), on_command, Command::ty());

    println!("Listening for commands...");

    listener.await
}
