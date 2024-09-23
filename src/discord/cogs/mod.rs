use crate::types::{Data, Error};

mod config;
mod misc;
mod stats;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        misc::boot(),
        stats::stats(),
        config::reaction_role::add_reaction_role(),
        config::get_config(),
        stats::leaderboard(),
    ]
}
