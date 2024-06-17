use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Fetch a gif from giphy.com"))]
pub async fn gif(
    ctx: Context<'_>,
    #[description = "Search gif by keyword"]
    search: String,
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching a gif from giphy.com").await.unwrap();

    let client = reqwest::Client::new();
    let token = env::var("GIPHY_TOKEN").expect("Expected giphy.com API in environment");

    // By default it will fetch 50 trending stickers
    let amount = 50;
    let url = format!("https://api.giphy.com/v1/gifs/search?api_key={}&q={}&limit={}", token, search, amount);
    
    let response = client.get(url).send().await.unwrap();
    let response_text = response.text().await.unwrap();
    
    let parsed_json: Value = serde_json::from_str(response_text.as_str()).unwrap();
    let random_index = rand::random::<usize>() % amount;
    let embed_url = parsed_json["data"][random_index]["embed_url"].as_str().unwrap();
    // let embed = CreateEmbed::new().url(embed_url);

    let create_reply = CreateReply {
        content: Some(embed_url.to_string()),
        ..Default::default()
    };

    fetching.edit(ctx, create_reply).await.unwrap();

    // let message = CreateMessage::new().content(embed_url);

    Ok(())
}