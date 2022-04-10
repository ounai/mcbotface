use std::env;
use futures::StreamExt;
use telegram_bot::*;

trait GetSender {
    fn get_sender(&self) -> String;
}

impl GetSender for Message {
    fn get_sender(&self) -> String {
        match &self.from {
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

async fn handle_command(message: &Message, text: String, api: &Api) -> Result<(), Error> {
    if text.starts_with("/hello") {
        api.send(message.text_reply("Hello!")).await?;
    }

    Ok(())
}

pub async fn run() -> Result<(), Error> {
    let token = env::var("TG_BOT_TOKEN")
        .expect("Environment variable TG_BOT_TOKEN must be set!");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("{}: {}", message.get_sender(), data);

                if data.starts_with("/") {
                    handle_command(&message, data.to_string(), &api).await?;
                }
            }
        }
    }

    Ok(())
}
