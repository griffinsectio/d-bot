use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Send a cute cat photo ><"))]
pub async fn cat(
    ctx: Context<'_>
) -> Result<(), Error> {
    let fetching = ctx.say("Finding a cute cat pic from thecatapi.com").await.unwrap();
    let client = reqwest::Client::new();
    let url = "https://api.thecatapi.com/v1/images/search";
    
    let response = client.get(url).send().await.unwrap();

    let body = response.text().await.unwrap();

    let result: Value = serde_json::from_str(body.as_str()).unwrap();

    let cat_url = Url::parse(result[0]["url"].as_str().unwrap()).unwrap();

    let downloaded_bytes = client.get(cat_url).send().await.unwrap().bytes().await.unwrap();

    std::fs::write("cat.jpg", downloaded_bytes).unwrap();

    let picture = CreateMessage::new()
        .content("Here's a picture of cat to brighten up your day :3")
        .add_file(CreateAttachment::path("./cat.jpg").await.unwrap());

    let msg = ctx.channel_id().send_message(&ctx.http(), picture).await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    } else {
        fetching.delete(ctx).await.unwrap();
        std::fs::remove_file("./cat.jpg").unwrap();
    }

    Ok(())
}