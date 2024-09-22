use crate::types::Error;

use poise::serenity_prelude as serenity;
use serenity::model::gateway::Ready;

pub async fn ready(_ctx: &serenity::Context, data_about_bot: &Ready) -> Result<(), Error> {
    println!("Logged in as {}", data_about_bot.user.name);

    Ok(())
}
