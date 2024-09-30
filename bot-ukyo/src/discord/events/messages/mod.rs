use crate::types::Error;
use poise::serenity_prelude::{self as serenity, CacheHttp, Message, UserId};

pub async fn message(ctx: &serenity::Context, new_message: &Message) -> Result<(), Error> {
    if new_message.author
        != ctx
            .http()
            .get_user(UserId::new(258772567433805824))
            .await
            .unwrap()
    {
        return Ok(());
    }

    if !new_message.content.to_uppercase().contains("AWESOME") {
        return Ok(());
    }

    new_message
        .channel_id
        .say(&ctx.http, "No you're awesome :heart:")
        .await?;

    Ok(())
}
