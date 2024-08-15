#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ReactionRole {
    pub message_id: u64,
    pub role_id: u64,
    pub emote_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Event {
    pub reaction_role: Option<Vec<ReactionRole>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
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
            events: Some(Event {
                reaction_role: Some(vec![ReactionRole {
                    message_id: 0,
                    role_id: 0,
                    emote_name: "heart_on_fire".to_string(),
                }]),
            }),
        }
    }
}
