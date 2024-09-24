use crate::{
    db::crud::{
        boot::create_boot,
        users::{create_user, get_user_by_discord_id},
    },
    types::{Context, Error},
};
use poise::serenity_prelude::Mentionable;
use rand::Rng;

#[poise::command(slash_command, guild_only)]
pub async fn boot(ctx: Context<'_>) -> Result<(), Error> {
    let message: String;
    let random_number: f64 = rand::thread_rng().gen();

    let user = ctx.author().mention();
    let user_id = ctx.author().id;

    if random_number >= 0.98 {
        message = format!(
            "<:Pepega:1286341573159948288> :mega: <@&1286347092289130568> <@{}> is a WHOPPING {}% booty",
            user,
            (random_number * 100.0).round()
        );
    } else if random_number <= 0.02 {
        message = format!(
            "<:Pepega:1286341573159948288> :mega: <@&1286347092289130568> <@{}> is ABSOLUTELY NOT BOOTY ({}%)",
            user,
            (random_number * 100.0).round()
        );
    } else if (random_number * 100.0).round() == 69.0 {
        message = format!(
            "<:Pepega:1286341573159948288> :mega: <@&1286347092289130568> <@{}> is a NICE booty ({}%)",
            user,
            (random_number * 100.0).round()
        );
    } else {
        message = format!("{} is {}% booty", user, (random_number * 100.0).round());
    }

    ctx.say(message).await.unwrap();

    if (get_user_by_discord_id(user_id.get()).await).is_none() {
        create_user(user_id.get()).await;
    };

    let user_db = get_user_by_discord_id(user_id.get()).await.unwrap();

    create_boot(user_db, random_number).await;

    Ok(())
}
