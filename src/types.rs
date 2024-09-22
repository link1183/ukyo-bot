use crate::config::Config;
use poise::Context as PoiseContext;
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

pub type Error = Box<dyn StdError + Send + Sync>;
pub type Context<'a> = PoiseContext<'a, Data, Error>;
