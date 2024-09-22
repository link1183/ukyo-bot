use crate::types::{Data, Error};

mod config;
mod misc;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![misc::boot(), config::add_reaction_role(), config::config()]
}
