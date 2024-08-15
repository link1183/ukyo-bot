use crate::types;
use poise::serenity_prelude::{self as serenity, ChannelId, CreateEmbed, CreateMessage};
use serenity::Member;

pub async fn join(ctx: &serenity::Context, new_member: &Member) -> Result<(), types::Error> {
    Ok(())
}
