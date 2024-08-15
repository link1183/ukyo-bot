use crate::types::{Data, Error};
use poise::serenity_prelude::{
    self as serenity, Channel, ChannelId, Context, CreateEmbed, CreateMessage,
};
use serenity::Member;

pub async fn join(ctx: &Context, data: &Data, new_member: &Member) -> Result<(), Error> {
    let config = data.config().clone();
    let events = config.events();

    if events.is_none() {
        return Ok(());
    }

    let events = events.clone().unwrap();

    if events.guild_join.is_none() {
        return Ok(());
    }

    let guild_join = events.guild_join.clone().unwrap();

    for i in guild_join {
        let channel_id = i.channel_id;
        let role_id = i.role_id;

        let channel = ChannelId::new(channel_id).to_channel(&ctx).await;

        if let Ok(Channel::Guild(channel)) = channel {
            if channel.guild_id != new_member.guild_id {
                continue;
            }
        }

        // let title = format!("Welcome {} to the discord server!", new_member.user.name);
        // let embed = CreateEmbed::default()
        //     .color(serenity::Colour::LIGHT_GREY)
        //     .title(title);
        //
        // let message = CreateMessage::new().embed(embed);
        //
        // let channel = ChannelId::new(channel_id);
        // let sent_message = channel.send_message(&ctx, message).await;
        //
        // if let Err(e) = sent_message {
        //     eprintln!("An error occured sending the message: {e}")
        // }

        if let Err(e) = new_member.add_role(ctx, role_id).await {
            eprintln!("Role with specified ID doesn't exist: {e}");
        };
    }

    Ok(())
}
