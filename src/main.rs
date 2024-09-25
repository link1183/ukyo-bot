mod config;
mod db;
mod discord;
mod types;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let cfg = config::load_config();

    if let Err(e) = discord::discord_bot(cfg).await {
        eprintln!("Error happened while handling the discord bot: {}", e);
    };

    Ok(())
}
