use poise::serenity_prelude::{self as serenity};

mod message;
mod reactions;
mod ready;
mod users;

use crate::types;

#[allow(unused)]
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, types::Data, types::Error>,
    data: &types::Data,
) -> Result<(), types::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            ready::ready(ctx, data_about_bot).await?;
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            reactions::reaction_add(ctx, data, add_reaction).await?;
        }
        serenity::FullEvent::ReactionRemove { removed_reaction } => {
            reactions::reaction_remove(ctx, data, removed_reaction).await?;
        }
        _ => {}
    }
    Ok(())
}
