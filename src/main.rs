mod discord;
use anyhow::Result;
mod config;
mod types;
mod utils;
use std::env;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let app_name = "ukyo-bot";

    let config_name: &str = if env::var("DEV").is_ok() {
        println!("Running in dev environnement");
        "dev"
    } else {
        println!("Running in production environnement");
        "config"
    };

    // Default config path: ~/.config/{app_name}/{config_name}.yml
    let cfg: config::Config = confy::load(app_name, config_name)?;

    if let Err(e) = discord::discord_bot(cfg).await {
        eprintln!("Error happened while handling the discord bot: {}", e);
    };

    Ok(())
}
