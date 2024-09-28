use crate::types::{Data, Error};

mod boot;
mod config;
mod messages;
mod stats;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        boot::boot(),
        stats::stats(),
        config::reaction_role::add_reaction_role(),
        config::get_config(),
        stats::board(),
        messages::submit(),
    ]
}
