use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Ping the bot"))]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Pong!").await.unwrap();
    Ok(())
}