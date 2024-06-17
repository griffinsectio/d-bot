use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Fetch an advice from adviceslip.com"))]
pub async fn advice(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // let msg = serenity::CreateMessage::new().content("Fetching an advice...");
    // let original_msg_id = ctx.channel_id().send_message(&ctx.http(), msg).await.unwrap().id;
    let fetching_advice = ctx.say("Fetching an advice...").await.unwrap();

    let client = reqwest::Client::new();
    let url = "https://api.adviceslip.com/advice";

    let response = client.get(url).send().await.unwrap();
    let response_json = response.text().await.unwrap();

    let result_json: serde_json::Value = serde_json::from_str(&response_json.as_str()).unwrap();
    let advice = result_json["slip"]["advice"].as_str().unwrap();

    let create_reply = CreateReply {
        content: Some(advice.to_string()),
        ..Default::default()
    };

    fetching_advice.edit(ctx, create_reply).await.unwrap();

    Ok(())
}