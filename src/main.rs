use std::env;

use futures::StreamExt;
use telegram_bot as tg;

fn get_sender(message: &tg::types::Message) -> String {
    match &message.from {
        // @username
        tg::User { username: Some(username), .. } =>
            ["@".to_string(), username.to_string()].concat(),

        // firstname lastname
        tg::User { first_name, last_name: Some(last_name), .. } =>
            [first_name.to_string(), last_name.to_string()].join(" "),

        // firstname
        tg::User { first_name, .. } =>
            first_name.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), tg::Error> {
    let token = env::var("TG_BOT_TOKEN")
        .expect("Environment variable TG_BOT_TOKEN must be set!");

    let api = tg::Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        if let tg::UpdateKind::Message(msg) = update.kind {
            if let tg::MessageKind::Text { ref data, .. } = msg.kind {
                let sender = get_sender(&msg);

                println!("<{}> {}", sender, data);
            }
        }
    }

    println!("Yes!");

    Ok(())
}
