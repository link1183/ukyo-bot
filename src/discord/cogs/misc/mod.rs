use poise::serenity_prelude as serenity;

use crate::types::{Context, Error};
use rand::Rng;

#[poise::command(slash_command, guild_only)]
pub async fn boot(ctx: Context<'_>, user: serenity::UserId) -> Result<(), Error> {
    let message: String;

    let random_number: f64 = rand::thread_rng().gen();

    if random_number >= 0.98 {
        message = format!(
            "<:Pepega:1286341573159948288> :mega: <@&1286347092289130568> <@{}> is a WHOPPING {}% booty",
            user,
            (random_number * 100.0).round()
        );
    } else if random_number <= 0.2 {
        message = format!(
            ":Pepega: :mega: <@&1286347092289130568> <@{}> is ABSOLUTELY NOT BOOTY ({}%)",
            user,
            (random_number * 100.0).round()
        );
    } else {
        message = format!("<@{}> is {}% booty", user, (random_number * 100.0).round());
    }

    ctx.say(message).await.unwrap();
    Ok(())
}
