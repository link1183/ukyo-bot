use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReactionRole {
    pub message_id: u64,
    pub role_id: u64,
    pub emote_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildJoin {
    pub channel_id: u64,
    pub role_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Event {
    pub reaction_role: Option<Vec<ReactionRole>>,
    pub guild_join: Option<Vec<GuildJoin>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    events: Option<Event>,
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
