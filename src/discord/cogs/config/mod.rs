use std::collections::HashMap;

use crate::config::{load_config, save_config, Event, ReactionRole};
use crate::types::{Context, Error};
use anyhow::Result;
use poise::serenity_prelude::{CreateEmbed, MessageId, RoleId};
use poise::CreateReply;
use regex::Regex;
use tokio::time::{sleep, Duration};
use unic_emoji_char::is_emoji;

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn add_reaction_role(
    ctx: Context<'_>,
    name: String,
    emote: String,
    role: RoleId,
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

    let role_found = roles.iter().find(|role_iter| role_iter.id == role.get());

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

    let new_reaction_role = ReactionRole::new(message_id as u64, role.get(), emote);
    let mut role_map: HashMap<String, ReactionRole> = HashMap::new();
    role_map.insert(name.clone(), new_reaction_role.clone());

    if let Some(events) = &mut cfg.events {
        if let Some(reaction_roles) = &mut events.reaction_role {
            reaction_roles.insert(name.clone(), new_reaction_role.clone());
        } else {
            events.reaction_role = Some(role_map);
        }
    } else {
        cfg.events = Some(Event {
            reaction_role: Some(role_map),
            guild_join: None,
        });
    }

    save_config(&cfg)?;

    ctx.say("ReactionRole added successfully").await.unwrap();

    Ok(())
}

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    let cfg = load_config();

    let mut embed = CreateEmbed::new().title("Configuration").color(0x00FF00);

    if let Some(events) = cfg.events {
        if let Some(reaction_roles) = events.reaction_role {
            // Fetch guild roles once and store them in a HashMap for quick lookup
            let roles = ctx
                .http()
                .get_guild_roles(ctx.guild_id().unwrap())
                .await
                .unwrap();
            let role_map: HashMap<u64, String> = roles
                .into_iter()
                .map(|role| (role.id.get(), role.name))
                .collect();

            for (name, reaction_role) in reaction_roles.iter() {
                let role_name = role_map
                    .get(&reaction_role.role_id)
                    .cloned()
                    .unwrap_or_else(|| "Role not found in the current guild".to_string());

                embed = embed.field(
                    name,
                    format!(
                        "Emote: {}\nRole name: {}\nMessage ID: {}",
                        reaction_role.emote_name, role_name, reaction_role.message_id,
                    ),
                    false,
                );
            }
        }
    } else {
        embed = embed.description("No events registered in this configuration");
    };

    let rep = CreateReply::default().embed(embed);

    ctx.send(rep).await?;

    Ok(())
}
