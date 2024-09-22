pub mod reaction_role;

use crate::config::load_config;
use crate::types::{Context, Error};
use anyhow::Result;
use poise::serenity_prelude::{ChannelId, CreateEmbed};
use poise::CreateReply;
use std::collections::HashMap;

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn get_config(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: Finish this
    let cfg = load_config();

    let mut roles_embed = CreateEmbed::new().title("Reaction roles").color(0x00FF00);
    let mut guild_embed = CreateEmbed::new().title("Guild joins").color(0x00FF00);

    let roles = ctx
        .http()
        .get_guild_roles(ctx.guild_id().unwrap())
        .await
        .unwrap();

    let role_map: HashMap<u64, String> = roles
        .into_iter()
        .map(|role| (role.id.get(), role.name))
        .collect();

    if let Some(events) = cfg.events {
        if let Some(reaction_roles) = events.reaction_role {
            for (name, reaction_role) in reaction_roles.iter() {
                let role_name = role_map
                    .get(&reaction_role.role_id)
                    .cloned()
                    .unwrap_or_else(|| "Role not found in the current guild".to_string());

                roles_embed = roles_embed.field(
                    name,
                    format!(
                        "Emote: {}\nRole name: {}\nMessage ID: {}",
                        reaction_role.emote_name, role_name, reaction_role.message_id,
                    ),
                    false,
                );
            }
        }

        if let Some(guild_join) = events.guild_join {
            for (index, join) in guild_join.iter().enumerate() {
                let role_name = role_map
                    .get(&join.role_id)
                    .cloned()
                    .unwrap_or_else(|| "Role not found in the current guild".to_string());

                let channel_name = ctx
                    .http()
                    .get_channel(ChannelId::new(join.channel_id))
                    .await
                    .unwrap()
                    .guild()
                    .unwrap()
                    .name;

                guild_embed = guild_embed.field(
                    format!("{}", index + 1),
                    format!("Role: {}\nChannel: {}", role_name, channel_name),
                    false,
                );
            }
        }
    } else {
        roles_embed = roles_embed.description("No events registered in this configuration");
    };

    let mut rep = CreateReply::default().embed(roles_embed);
    rep = rep.embed(guild_embed);

    ctx.send(rep).await?;

    Ok(())
}
