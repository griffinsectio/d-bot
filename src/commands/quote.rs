use crate::*;

#[derive(Debug, poise::ChoiceParameter)]
enum QuoteTopic {
    #[name = "Learning"]
    Learning,
    #[name = "Intelligence"]
    Intelligence,
    #[name = "Knowledge"]
    Knowledge,
    #[name = "Leadership"]
    Leadership,
    #[name = "Success"]
    Success,
}

#[poise::command(slash_command, description_localized("en-US", "Fetch a quote from api-ninjas.com"))]
pub async fn quote(
    ctx: Context<'_>,
    topic: QuoteTopic,
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching a quote from apininjas.com").await.unwrap();

    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");

    let client = reqwest::Client::new();
    let url = format!("https://api.api-ninjas.com/v1/quotes?category={}", topic.name().to_lowercase());
    
    let response = client.get(url).header("X-Api-Key", token).send().await.unwrap();

    let body = response.text().await.unwrap();

    let result: Value = serde_json::from_str(body.as_str()).unwrap();

    let quote = format!("\"{}\"", result[0]["quote"].as_str().unwrap());
    let author = format!("\\- {}", result[0]["author"].as_str().unwrap());

    let content = format!("{}\n{}", quote, author);

    let edited_message = CreateReply {
        content: Some(content),
        ..Default::default()
    };

    if let Err(why) = fetching.edit(ctx, edited_message).await {
        println!("Error sending message: {}", why);
    }

    Ok(())
}