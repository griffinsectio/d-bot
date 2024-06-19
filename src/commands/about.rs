use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Get information about user"))]
pub async fn about(
    ctx: Context<'_>,
    user: serenity::User,
) -> Result<(), Error> {
    let id = user.id.to_string();
    let name = user.clone().name;
    let global_name = match user.clone().global_name {
        Some(v) => v,
        None => "No global name".to_string()
    };
    let tag = user.tag();
    
    let created_at = user.created_at().to_utc();
    let created_at_date = created_at.format("%d-%m-%Y");
    let created_at_time = created_at.format("%H:%M:%S");
    let created_at_info = format!("{} {} UTC", created_at_date, created_at_time);

    let message = format!(
        "The user's id is: {}\nThe user's name is: {}\nThe user's global name is: {}\nThe user's tag is: {}\nThe user's account created at {}"
        , id, name, global_name, tag, created_at_info
    );

    ctx.say(message).await.unwrap();

    Ok(())
}