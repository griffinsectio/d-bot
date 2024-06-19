use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Give a riddle"))]
pub async fn riddle(
    ctx: Context<'_>
) -> Result<(), Error> {
    let thinking = ctx.say("Thinking of a riddle...").await.unwrap();

    let client = reqwest::Client::new();
    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");
    let response = client.get("https://api.api-ninjas.com/v1/riddles").header("X-Api-Key", token).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let result: Value = serde_json::from_str(&response_text.as_str()).unwrap();

    let title = &result[0]["title"].as_str().unwrap();
    let question = &result[0]["question"].as_str().unwrap();
    let answer = &result[0]["answer"].as_str().unwrap();
    let spoiler_answer = format!("||{}||", answer);

    let riddle_string = format!("Title: {}\nQuestion: {}\nAnswer: {}", title, question, spoiler_answer);

    let riddle = CreateReply {
        content: Some(riddle_string),
        ..Default::default()
    };

    if let Err(why) = thinking.edit(ctx, riddle).await {
        println!("Error sending message: {}", why);
    }

    Ok(())
}