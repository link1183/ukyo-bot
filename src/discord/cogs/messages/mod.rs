use crate::{
    db::{
        crud::{
            messages::create_message,
            users::{create_user, get_user_by_discord_id},
        },
        get_connection,
    },
    types::{Context, Error},
};

#[poise::command(slash_command, guild_only)]
pub async fn submit(ctx: Context<'_>, message: String) -> Result<(), Error> {
    let conn = get_connection().await;
    let user = get_user_by_discord_id(conn.clone(), ctx.author().id.get()).await;

    if user.is_none() {
        let new_user = create_user(conn.clone(), ctx.author().id.get()).await;
        create_message(conn.clone(), new_user, message.clone()).await;
    } else {
        create_message(conn.clone(), user.unwrap(), message.clone()).await;
    };

    ctx.say(format!(
        "The welcome message \"{}\" was successfully registered",
        &message
    ))
    .await?;

    Ok(())
}
