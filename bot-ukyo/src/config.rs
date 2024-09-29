use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReactionRole {
    pub message_id: u64,
    pub role_id: u64,
    pub emote_name: String,
}

impl ReactionRole {
    pub fn new(message_id: u64, role_id: u64, emote_name: String) -> Self {
        ReactionRole {
            message_id,
            role_id,
            emote_name,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildJoin {
    pub channel_id: u64,
    pub role_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Event {
    pub reaction_role: Option<HashMap<String, ReactionRole>>,
    pub guild_join: Option<Vec<GuildJoin>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    pub events: Option<Event>,
}

impl Config {
    pub fn events(&self) -> &Option<Event> {
        &self.events
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord_token: "".to_string(),
            events: None,
        }
    }
}

pub fn load_config() -> Config {
    let app_name = "ukyo-bot";

    let config_name: &str = if env::var("DEV").is_ok() {
        "dev"
    } else {
        "config"
    };

    // Default config path: ~/.config/{app_name}/{config_name}.yml
    let cfg: Config = confy::load(app_name, config_name).unwrap();
    dbg!(&cfg);
    cfg
}

pub fn save_config(cfg: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app_name = "ukyo-bot";

    let config_name: &str = if env::var("DEV").is_ok() {
        "dev"
    } else {
        "config"
    };

    confy::store(app_name, config_name, cfg)?;

    Ok(())
}
