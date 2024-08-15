use crate::config::Config;
use crate::types::{self, Error};
use poise::serenity_prelude::{self as serenity, GatewayIntents};
use std::env;

mod events;

pub async fn discord_bot(config: Config) -> Result<(), Error> {
    let intents = if env::var("DEV").is_ok() {
        GatewayIntents::all()
    } else {
        GatewayIntents::from_bits_truncate(1719528745463031)
    };

    let options = poise::FrameworkOptions {
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
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                let data = types::Data {
                    config: config_clone,
                };
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
