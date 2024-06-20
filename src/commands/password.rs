use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Generate a random password"))]
pub async fn password(
    ctx: Context<'_>,
    length: u8,
    exclude_numbers: bool,
    exclude_special_chars: bool,
) -> Result<(), Error> {
    let thinking = ctx.say("Generating random password...").await.unwrap();

    let url = format!("https://api.api-ninjas.com/v1/passwordgenerator?length={}&exclude_numbers={}&exclude_special_chars={}", length, exclude_numbers, exclude_special_chars);

    let client = reqwest::Client::new();
    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");
    let response = client.get(url).header("X-Api-Key", token).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let result: Value = serde_json::from_str(response_text.as_str()).unwrap();

    let dm = CreateReply {
        content: Some("Check your dm".to_string()),
        ..Default::default()
    };

    let random_password = result["random_password"].as_str().unwrap();
    let message = CreateMessage::new()
    .content(random_password);

    if let Err(_) = ctx.author().dm(&ctx.http(), message).await {
        ctx.say(format!("Unable to dm {}", ctx.author().name)).await.unwrap();
    };

    if let Err(why) = thinking.edit(ctx, dm).await {
        println!("Error sending message: {}", why);
    }

    Ok(())
}