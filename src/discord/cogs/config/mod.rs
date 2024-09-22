use crate::config::{load_config, save_config, Event, ReactionRole};
use crate::types::{Context, Error};
use poise::serenity_prelude::MessageId;
use regex::Regex;
use tokio::time::{sleep, Duration};
use unic_emoji_char::is_emoji;

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn add_reaction_role(
    ctx: Context<'_>,
    emote: String,
    role_id: u128,
    message_id: u128,
) -> Result<(), Error> {
    let mut cfg = load_config();

    // Regular expression to match custom Discord emojis (animated and non-animated)
    let custom_emoji_re = Regex::new(r"^<a?:\w+:\d+>$").unwrap();

    // Regular expression to match complex Unicode emoji sequences (including ZWJ sequences)
    let complex_unicode_emoji_re = Regex::new(r"^(\p{Emoji_Presentation}|\p{Emoji}\uFE0F)(\u200D(\p{Emoji_Presentation}|\p{Emoji}\uFE0F))*$").unwrap();

    let is_valid_emoji = custom_emoji_re.is_match(&emote)
        || complex_unicode_emoji_re.is_match(&emote)
        || is_emoji(emote.chars().next().unwrap());

    if !is_valid_emoji {
        let msg = ctx.say("Please enter a valid emoji").await.unwrap();
        sleep(Duration::from_secs(5)).await;
        msg.delete(ctx).await?;
        return Ok(());
    }

    let roles = ctx
        .http()
        .get_guild_roles(ctx.guild_id().unwrap())
        .await
        .unwrap();

    let role_found = roles.iter().find(|role| role.id == role_id as u64);

    if role_found.is_none() {
        let msg = ctx.say("Please enter a valid role ID").await?;
        sleep(Duration::from_secs(5)).await;
        msg.delete(ctx).await?;

        return Ok(());
    }

    match ctx
        .http()
        .get_message(ctx.channel_id(), MessageId::new(message_id as u64))
        .await
    {
        Ok(msg) => msg,
        Err(_) => {
            let msg = ctx.say("Please provide a valid message ID").await.unwrap();
            sleep(Duration::from_secs(5)).await;
            msg.delete(ctx).await.unwrap();
            return Ok(());
        }
    };

    let new_reaction_role = ReactionRole::new(message_id as u64, role_id as u64, emote);

    if let Some(events) = &mut cfg.events {
        if let Some(reaction_roles) = &mut events.reaction_role {
            reaction_roles.push(new_reaction_role);
        } else {
            events.reaction_role = Some(vec![new_reaction_role]);
        }
    } else {
        cfg.events = Some(Event {
            reaction_role: Some(vec![new_reaction_role]),
            guild_join: None,
        });
    }

    save_config(&cfg)?;

    ctx.say("ReactionRole added successfully").await.unwrap();

    Ok(())
}
