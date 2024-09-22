use crate::{config::load_config, types::Error};
use poise::serenity_prelude::{self as serenity, Reaction};
use regex::Regex;

fn parse_unicode_string(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some('u') = chars.next() {
                if let Some('{') = chars.next() {
                    let mut unicode_escape = String::new();
                    for d in chars.by_ref() {
                        if d == '}' {
                            break;
                        }
                        unicode_escape.push(d);
                    }
                    if let Ok(codepoint) = u32::from_str_radix(&unicode_escape, 16) {
                        if let Some(unicode_char) = char::from_u32(codepoint) {
                            result.push(unicode_char);
                        }
                    }
                    continue;
                }
            }
        }
        result.push(c);
    }

    result
}

pub async fn reaction_add(ctx: &serenity::Context, add_reaction: &Reaction) -> Result<(), Error> {
    let config = load_config();
    let events = config.events();

    if events.is_none() {
        return Ok(());
    }

    let events = events.clone().unwrap();

    if events.reaction_role.is_none() {
        return Ok(());
    }

    let reaction_roles = events.clone().reaction_role.unwrap();
    let regex_pattern = r"<:[a-zA-Z]+:\d+:>";
    let re = Regex::new(regex_pattern).unwrap();

    for (_, i) in reaction_roles {
        let reaction_message = i.message_id;
        let role_id = i.role_id;
        let mut emote_name = i.emote_name;

        if !re.is_match(emote_name.as_str()) {
            emote_name = parse_unicode_string(emote_name.as_str());
        }

        if add_reaction.message_id != reaction_message {
            continue;
        }

        if add_reaction.emoji.to_string() != emote_name {
            continue;
        }

        let guild_id = match add_reaction.guild_id {
            Some(t) => t,
            None => continue,
        };

        let user_id = add_reaction
            .user_id
            .expect("Expected a user ID in reaction event.");
        let role_id = serenity::RoleId::new(role_id);

        let guild_id = match guild_id.member(&ctx, user_id).await {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        };

        if let Err(e) = guild_id.add_role(&ctx, role_id).await {
            eprintln!("Failed to add role: {}", e);
        }
    }

    Ok(())
}

pub async fn reaction_remove(
    ctx: &serenity::Context,
    removed_reaction: &Reaction,
) -> Result<(), Error> {
    let config = load_config();
    let events = config.events();

    if events.is_none() {
        return Ok(());
    }

    let events = events.clone().unwrap();

    if events.reaction_role.is_none() {
        return Ok(());
    }

    let reaction_roles = events.clone().reaction_role.unwrap();

    let regex_pattern = r"<:[a-zA-Z]+:\d+:>";
    let re = Regex::new(regex_pattern).unwrap();

    for (_, i) in reaction_roles {
        let reaction_message = i.message_id;
        let role_id = i.role_id;
        let mut emote_name = i.emote_name;

        if !re.is_match(emote_name.as_str()) {
            emote_name = parse_unicode_string(emote_name.as_str());
        }

        if removed_reaction.message_id != reaction_message {
            continue;
        }

        if removed_reaction.emoji.to_string() != emote_name {
            continue;
        }

        let guild_id = match removed_reaction.guild_id {
            Some(t) => t,
            None => continue,
        };

        let user_id = removed_reaction
            .user_id
            .expect("Expected a user ID in reaction event.");
        let role_id = serenity::RoleId::new(role_id);

        let guild_id = match guild_id.member(&ctx, user_id).await {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        };

        if let Err(e) = guild_id.remove_role(&ctx, role_id).await {
            eprintln!("Failed to remove role: {}", e);
        }
    }

    Ok(())
}
