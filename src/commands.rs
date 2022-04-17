use std::error::Error;

use teloxide::{
    prelude2::*,
    utils::command::BotCommand,
    types::User,
};

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "send a hello message")]
    Hello,
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

async fn on_command(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Hello =>
            bot.send_message(message.chat.id, get_hello(&message.get_sender())).await?,
    };

    Ok(())
}

pub async fn listen(bot: &'static AutoSend<Bot>) {
    println!("Initializing command listeners...");

    let listener = teloxide::repls2::commands_repl(bot, on_command, Command::ty());

    println!("Listening for commands...");

    listener.await
}
