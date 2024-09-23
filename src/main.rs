mod discord;
use anyhow::Result;
use db::crud::users::get_user_by_discord_id;
mod config;
mod db;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let cfg = config::load_config();

    get_user_by_discord_id(1).await;

    if let Err(e) = discord::discord_bot(cfg).await {
        eprintln!("Error happened while handling the discord bot: {}", e);
    };

    Ok(())
}
