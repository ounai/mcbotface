use std::env;
use futures::StreamExt;

use telegram_bot::{
    CanReplySendMessage,
    Api,
    UpdateKind,
    MessageKind,
    Error as TGError
};

mod command;
use command::Command;

async fn handle_command(cmd: Command, api: &Api) -> Result<(), TGError> {
    println!("{}", cmd);

    if cmd.starts_with("/hello") {
        api.send(cmd.get_message().text_reply("Hello!")).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), TGError> {
    let token = env::var("TG_BOT_TOKEN")
        .expect("Environment variable TG_BOT_TOKEN must be set!");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if data.starts_with("/") {
                    let text = data.to_string();
                    let cmd = Command::new(message, text);

                    handle_command(cmd, &api).await?;
                }
            }
        }
    }

    Ok(())
}
