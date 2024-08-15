mod discord;
use anyhow::Result;
mod config;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let app_name = "ukyo-bot";
    let config_name = "config";

    // Default config path: ~/.config/{app_name}/{config_name}.yml
    let cfg: config::Config = confy::load(app_name, config_name)?;

    if let Err(e) = discord::discord_bot(cfg).await {
        eprintln!("Error happened while handling the discord bot: {}", e);
    };

    Ok(())
}
