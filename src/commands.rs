use std::error::Error;

use teloxide::{
    prelude2::*,
    utils::command::BotCommand,
    types::{User, ParseMode},
};

use crate::tj;

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

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "send a hello message")]
    Hello,

    #[command(description = "count days to a date")]
    Tj,

    #[command(description = "get a user & group id")]
    Id,
}

async fn on_command(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Hello =>
            bot.send_message(message.chat.id, format!("Hello, {}!", message.get_sender()))
                .reply_to_message_id(message.id)
                .await?,

        Command::Tj =>
            bot.send_message(message.chat.id, tj::get_days_until("2022-07-01"))
                .reply_to_message_id(message.id)
                .await?,

        Command::Id =>
            bot.send_message(message.chat.id, format!("user <code>{}</code>\ngroup <code>{}</code>", message.from().unwrap().id, message.chat.id))
                .parse_mode(ParseMode::Html)
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
