use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Throw a joke"))]
pub async fn joke(
    ctx: Context<'_>
) -> Result<(), Error> {
    let fetching = ctx.say("Thinking of a joke...").await.unwrap();
    let client = reqwest::Client::new();
    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");
    let response = client.get("https://api.api-ninjas.com/v1/jokes?").header("X-Api-Key", token).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let parse_result: Value = serde_json::from_str(response_text.as_str()).unwrap();
    let content = parse_result[0]["joke"].as_str().unwrap();

    let edited_message = CreateReply {
        content: Some(content.to_string()),
        ..Default::default()
    };

    if let Err(why) = fetching.edit(ctx, edited_message).await {
        println!("Error sending message: {why:?}");
    }

    Ok(())
}