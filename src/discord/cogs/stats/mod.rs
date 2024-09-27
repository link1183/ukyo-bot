use std::{cmp::Ordering, collections::HashMap};

use poise::{
    serenity_prelude::{self as serenity, CreateEmbed, Mentionable, UserId},
    CreateReply,
};

use crate::{
    db::{
        crud::boot::{get_all_boots_by_discord_id, get_leaderboard},
        get_connection,
    },
    types::{Context, Error},
};

#[derive(poise::ChoiceParameter)]
enum LeaderboardOrder {
    #[name = "winner"]
    Winner,
    #[name = "loser"]
    Loser,
}

#[poise::command(slash_command, guild_only)]
pub async fn board(ctx: Context<'_>, order: LeaderboardOrder) -> Result<(), Error> {
    let conn = get_connection().await;
    let lb = get_leaderboard(conn).await;

    // TODO: Process the leaderboard object based on the order
    let mut scores_by_user: HashMap<u64, Vec<f64>> = HashMap::new();
    for entry in lb {
        scores_by_user
            .entry(entry.discord_id)
            .or_default()
            .push(entry.score);
    }

    let title = match order {
        LeaderboardOrder::Winner => "Booty leaderboard",
        LeaderboardOrder::Loser => "Booty loserboard",
    };

    // Highest/lowest score calculation
    let mut score = String::new();
    for (i, (discord_id, scores)) in scores_by_user.iter().enumerate() {
        // Highest/lowest score
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let top_scores: Vec<f64> = match order {
            LeaderboardOrder::Winner => sorted_scores.iter().take(10).cloned().collect(),
            LeaderboardOrder::Loser => sorted_scores.iter().rev().take(10).cloned().collect(),
        };

        let username = ctx.http().get_user(UserId::new(*discord_id)).await.unwrap();
        let used_score: f64 = match order {
            LeaderboardOrder::Winner => top_scores.iter().cloned().fold(f64::MIN, f64::max),
            LeaderboardOrder::Loser => top_scores.iter().cloned().fold(f64::MAX, f64::min),
        };
        score.push_str(&format!(
            "{}. {}\n**{}%**\n",
            i + 1,
            username,
            (used_score * 100.0).round()
        ));
    }

    let mut average_score_array: HashMap<u64, f64> = HashMap::new();
    for (discord_id, scores) in scores_by_user {
        let average_score = scores.iter().sum::<f64>() / scores.len() as f64;
        average_score_array.insert(discord_id, average_score);
    }

    let mut sorted_avg_scores: Vec<(u64, f64)> = average_score_array.into_iter().collect();
    sorted_avg_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let avg_scores: Vec<(u64, f64)> = match order {
        LeaderboardOrder::Winner => sorted_avg_scores.iter().take(10).cloned().collect(),
        LeaderboardOrder::Loser => sorted_avg_scores.iter().rev().take(10).cloned().collect(),
    };

    let mut avg = String::new();
    for (i, (discord_id, average_score)) in avg_scores.iter().enumerate() {
        let username = ctx.http().get_user(UserId::new(*discord_id)).await.unwrap();
        avg.push_str(&format!(
            "{}. {}\n**{}%**\n",
            i + 1,
            username,
            (average_score * 10000.0).round() / 100.0
        ));
    }

    let embed = CreateEmbed::default()
        .title(title)
        .color(0x00FFFF)
        .field("Score", score, true)
        .field("Average", avg, true);

    let rep = CreateReply::default().embed(embed);

    ctx.send(rep).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn stats(ctx: Context<'_>, user: Option<serenity::UserId>) -> Result<(), Error> {
    let conn = get_connection().await;
    let discord_id = match user {
        Some(u) => u.get(),
        None => ctx.author().id.get(),
    };

    let username = ctx.http().get_user(UserId::new(discord_id)).await.unwrap();

    let boots = get_all_boots_by_discord_id(conn, discord_id).await;

    if boots.is_none() {
        ctx.say("No score registered for that user").await.unwrap();
        return Ok(());
    }

    let boots = boots.unwrap();

    let number_of_69: Vec<_> = boots
        .iter()
        .filter(|boot| (boot.score * 100.0).round() == 69.0)
        .collect();

    let total_score: f64 = boots.iter().map(|boot| (boot.score * 100.0).round()).sum();

    let average_score = (total_score.round() / (boots.len() as f64) * 100.0).round() / 100.0;
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

    let embed = CreateEmbed::default()
        .title("Booty stats")
        .description(format!("{}'s stats", username.mention()))
        .color(0x00FF00)
        .field("Count", count.to_string(), false)
        .field("Lowest boot", format!("{}%", min), false)
        .field("Highest boot", format!("{}%", max), false)
        .field("Average boot", format!("{}%", average_score), false)
        .field("Number of 69", number_of_69.len().to_string(), false);

    let rep = CreateReply::default().embed(embed);

    ctx.send(rep).await?;

    Ok(())
}
