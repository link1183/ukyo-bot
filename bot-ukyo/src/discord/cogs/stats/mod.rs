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
pub async fn lb(ctx: Context<'_>, order: LeaderboardOrder) -> Result<(), Error> {
    let conn = get_connection().await;
    let lb = get_leaderboard(conn).await;

    let mut scores_by_user: HashMap<u64, Vec<i32>> = HashMap::new();
    for entry in lb {
        scores_by_user
            .entry(entry.discord_id)
            .or_default()
            .push((entry.score * 100.0).round() as i32);
    }

    let (title, is_winner) = match order {
        LeaderboardOrder::Winner => ("Booty leaderboard", true),
        LeaderboardOrder::Loser => ("Booty loserboard", false),
    };

    let mut sorted_users: Vec<_> = scores_by_user.into_iter().collect();
    sorted_users.sort_by(|(_, a_scores), (_, b_scores)| {
        let a_score = if is_winner {
            *a_scores.iter().max().unwrap()
        } else {
            *a_scores.iter().min().unwrap()
        };
        let b_score = if is_winner {
            *b_scores.iter().max().unwrap()
        } else {
            *b_scores.iter().min().unwrap()
        };
        if is_winner {
            b_score.cmp(&a_score)
        } else {
            a_score.cmp(&b_score)
        }
    });

    let mut score = String::new();
    let mut avg = String::new();

    let mut avg_sorted_users = sorted_users.clone();
    avg_sorted_users.sort_by(|(_, a_scores), (_, b_scores)| {
        let a_avg = a_scores.iter().sum::<i32>() as f64 / a_scores.len() as f64;
        let b_avg = b_scores.iter().sum::<i32>() as f64 / b_scores.len() as f64;
        if is_winner {
            b_avg.partial_cmp(&a_avg).unwrap()
        } else {
            a_avg.partial_cmp(&b_avg).unwrap()
        }
    });

    for (i, (discord_id, scores)) in sorted_users.iter().take(10).enumerate() {
        let username = ctx.http().get_user(UserId::new(*discord_id)).await?;
        let used_score = if is_winner {
            *scores.iter().max().unwrap()
        } else {
            *scores.iter().min().unwrap()
        };

        score.push_str(&format!("{}. {}\n**{}%**\n", i + 1, username, used_score));
    }

    for (i, (discord_id, scores)) in avg_sorted_users.iter().take(10).enumerate() {
        let username = ctx.http().get_user(UserId::new(*discord_id)).await?;
        let average_score =
            (scores.iter().sum::<i32>() as f64 / scores.len() as f64 * 100.0).round() / 100.0;

        avg.push_str(&format!(
            "{}. {}\n**{:.2}%**\n",
            i + 1,
            username,
            average_score
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

    let average_score = total_score.round() / boots.len() as f64;
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
        .field("Average boot", format!("{:.2}%", average_score), false)
        .field("Number of 69", number_of_69.len().to_string(), false);

    let rep = CreateReply::default().embed(embed);

    ctx.send(rep).await?;

    Ok(())
}
