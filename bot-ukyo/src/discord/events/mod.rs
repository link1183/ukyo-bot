use poise::serenity_prelude::{self as serenity, FullEvent};

mod messages;
mod reactions;
mod ready;
mod users;

use crate::types;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, types::Data, types::Error>,
    data: &types::Data,
) -> Result<(), types::Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            ready::ready(ctx, data_about_bot).await?;
        }
        FullEvent::ReactionAdd { add_reaction } => {
            reactions::reaction_add(ctx, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            reactions::reaction_remove(ctx, removed_reaction).await?;
        }
        FullEvent::GuildMemberAddition { new_member } => {
            users::join(ctx, data, new_member).await?;
        }
        FullEvent::Message { new_message } => {
            messages::message(ctx, new_message).await?;
        }
        _ => {}
    }
    Ok(())
}
