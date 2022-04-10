use telegram_bot as tg;

mod bot;

#[tokio::main]
async fn main() -> Result<(), tg::Error> {
    println!("Initializing...");

    bot::run().await?;

    Ok(())
}
