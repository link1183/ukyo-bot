use crate::config::Config;
use crate::types::{self, Error};
use poise::serenity_prelude::{self as serenity, GatewayIntents, GuildId};
use std::env;

mod cogs;
mod events;

pub async fn discord_bot(config: Config) -> Result<(), Error> {
    let intents = if env::var("DEV").is_ok() {
        GatewayIntents::all()
    } else {
        GatewayIntents::from_bits_truncate(1719528745463031)
    };

    let options = poise::FrameworkOptions {
        commands: cogs::get_commands(),
        event_handler: |ctx, event, framework, data| {
            Box::pin(async move {
                match events::event_handler(ctx, event, framework, data).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            })
        },
        ..Default::default()
    };

    let config_clone = config.clone();

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let guild_id: u64 = env::var("GUILD_ID")
                    .expect("GUILD_ID must be set")
                    .parse()
                    .expect("GUILD_ID must be a valid u64");

                let guild = GuildId::new(guild_id);

                let data = types::Data {
                    config: config_clone,
                };

                poise::builtins::register_in_guild(&ctx, &framework.options().commands, guild)
                    .await?;
                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(&config.discord_token, intents)
        .framework(framework)
        .await
        .unwrap();

    match client.start().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
