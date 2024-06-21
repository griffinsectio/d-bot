use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Throw a dad joke"))]
pub async fn dadjoke(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let thinking = ctx.say("Thinking of a dad joke...").await.unwrap();

    let url = "https://api.api-ninjas.com/v1/dadjokes";

    let client = reqwest::Client::new();
    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");
    let response = client.get(url).header("X-Api-Key", token).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let result: Value = serde_json::from_str(response_text.as_str()).unwrap();
    let joke = CreateReply {
        content: Some(result[0]["joke"].as_str().unwrap().to_string()),
        ..Default::default()
    };

    if let Err(why) = thinking.edit(ctx, joke).await {
        println!("Error sending message: {}", why);
    }

    Ok(())
}