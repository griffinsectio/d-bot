use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Fetch random image from pexels.com"))]
pub async fn image(
    ctx: Context<'_>,
    #[description = "Topic of the random image"]
    topic: String
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching an image from pexels.com").await.unwrap();

    let pictures_amount = 50;

    let client = reqwest::Client::new();
    let url = format!("https://api.pexels.com/v1/search?query={}&per_page={}", topic, pictures_amount);
    let token = env::var("PEXELS_TOKEN").expect("Expected pexels.com token in environment");
    let response = client.get(url).header("Authorization", token).send().await.unwrap();

    let response_text = response.text().await.unwrap();
    let parsed_json: Value = serde_json::from_str(response_text.as_str()).unwrap();


    let images = parsed_json["photos"].as_array().unwrap();

    if images.len() == 0 {
        ctx.say("Unable to find any image...").await.unwrap();
        return Ok(());
    }

    let random_index = rand::random::<usize>() % images.len();
    let image_url = Url::parse(images[random_index]["src"]["original"].as_str().unwrap()).unwrap();
    let image_bytes = client.get(image_url.clone()).send().await.unwrap().bytes().await.unwrap();
    std::fs::write("./image.jpeg", image_bytes).unwrap();

    let picture = CreateMessage::new()
    .content("Here's the image I found from pexels.com")
    .add_file(CreateAttachment::path("./image.jpeg").await.unwrap());

    let msg = ctx.channel_id().send_message(&ctx.http(), picture).await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    } else {
        fetching.delete(ctx).await.unwrap();
        std::fs::remove_file("./image.jpeg").unwrap();
    }

    Ok(())
}