use poise::serenity_prelude as serenity;

use crate::types::{Context, Error};
use rand::Rng;

#[poise::command(slash_command, guild_only)]
pub async fn gay(ctx: Context<'_>, user: serenity::UserId) -> Result<(), Error> {
    let random_number: f64 = rand::thread_rng().gen();

    let message = format!("<@{}> is {}% gay", user, (random_number * 100.0).round());
    ctx.say(message).await.unwrap();
    Ok(())
}
