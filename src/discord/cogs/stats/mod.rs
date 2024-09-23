use std::cmp::Ordering;

use poise::serenity_prelude as serenity;

use crate::{
    db::crud::{boot::get_all_boots_by_discord_id, users::get_all_users},
    types::{Context, Error},
};

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn get_users(ctx: Context<'_>) -> Result<(), Error> {
    let users = get_all_users().await;

    dbg!(users);

    ctx.say("Hey").await.unwrap();

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn stats(ctx: Context<'_>, user: Option<serenity::UserId>) -> Result<(), Error> {
    let discord_id = match user {
        Some(u) => u.get(),
        None => ctx.author().id.get(),
    };

    let boots = get_all_boots_by_discord_id(discord_id).await;

    if boots.is_none() {
        ctx.say("No score registered for that user").await.unwrap();
        return Ok(());
    }

    let boots = boots.unwrap();

    let total_score: f64 = boots.iter().map(|boot| (boot.score * 100.0).round()).sum();

    dbg!(total_score);

    let average_score = total_score.round() / (boots.len() as f64);
    let min = (boots
        .iter()
        .min_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal))
        .map(|boot| boot.score)
        .unwrap()
        * 100.0)
        .round();
    let max = (boots
        .iter()
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal))
        .map(|boot| boot.score)
        .unwrap()
        * 100.0)
        .round();
    let count = boots.len();

    let msg = format!(
        "Count: {}\nMin: {}%\nMax: {}%\nAverage: {}%",
        count, min, max, average_score
    );

    ctx.say(msg).await.unwrap();

    Ok(())
}
