use crate::types::{Data, Error};
use poise::serenity_prelude::{
    self as serenity, Channel, ChannelId, Context, CreateEmbed, CreateMessage,
};
use rand::Rng;
use serenity::Member;

pub async fn join(ctx: &Context, data: &Data, new_member: &Member) -> Result<(), Error> {
    let config = data.config().clone();
    let events = config.events();

    if events.is_none() {
        return Ok(());
    }

    // Can safely unwrap there
    let events = events.clone().unwrap();

    if events.guild_join.is_none() {
        return Ok(());
    }

    let guild_join = events.guild_join.clone().unwrap();

    for i in guild_join {
        // TODO: Implement the messages from the database there
        let join_message = "Welcome {user} to the boots gang";

        let channel_id = i.channel_id;
        let role_id = i.role_id;

        let channel = ChannelId::new(channel_id).to_channel(&ctx).await;

        if let Ok(Channel::Guild(channel)) = channel {
            if channel.guild_id != new_member.guild_id {
                continue;
            }
        }

        // Forced to gen the RNG in a block like this as it messes up with other stuff
        let (red, green, blue) = tokio::task::spawn_blocking(|| {
            let mut rng = rand::thread_rng();
            let red = rng.gen_range(0..255);
            let green = rng.gen_range(0..255);
            let blue = rng.gen_range(0..255);
            (red, green, blue)
        })
        .await?;

        let title = join_message.replace("{user}", new_member.display_name());

        let embed = CreateEmbed::default()
            .color(serenity::Colour::from_rgb(red, green, blue))
            .title(title);

        let message = CreateMessage::new().embed(embed);

        let channel = ChannelId::new(channel_id);
        channel.send_message(&ctx, message).await?;

        // TODO: Implement logging
        new_member.add_role(ctx, role_id).await?;
    }

    Ok(())
}
