use crate::config::Config;
use poise::Context as PoiseContext;
use sea_orm::FromQueryResult;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Data {
    pub config: Config,
}

impl Data {
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[derive(Debug, FromQueryResult)]
pub struct Leaderboard {
    pub discord_id: u64,
    pub score: f64,
}

pub type Error = Box<dyn StdError + Send + Sync>;
pub type Context<'a> = PoiseContext<'a, Data, Error>;
