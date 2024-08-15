use crate::types::{Data, Error};
use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;

pub async fn message(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<(), Error> {
    if new_message.author.id == framework.bot_id {
        return Ok(());
    }

    let channel = new_message.channel_id.name(&ctx).await?;
    let guild_id = match new_message.guild_id {
        Some(t) => t,
        None => return Ok(()), // Message was not give over the gateway
    };

    let guild = match guild_id.name(ctx) {
        Some(t) => t,
        None => return Ok(()), // Message was sent in a DM
    };

    println!(
        "[{}.{}] {}: {}",
        guild,
        channel,
        new_message.author.name,
        new_message.content_safe(ctx)
    );

    Ok(())
}
