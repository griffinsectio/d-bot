mod commands;

use std::env;

use serde_json::{self, Value};

use joketeller::{
    Joker, Category,
};

use reqwest::Url;

use serenity::all::CreateMessage;
use serenity::builder::CreateAttachment;

use poise::{serenity_prelude as serenity, ChoiceParameter, CreateReply};


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, description_localized("en-US", "Ping the bot"))]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Pong!").await.unwrap();
    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Fetch an advice from adviceslip.com"))]
async fn advice(
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

#[poise::command(slash_command, description_localized("en-US", "Fetch a sticker from giphy.com"))]
async fn sticker(
    ctx: Context<'_>
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching a sticker from giphy.com").await.unwrap();

    let client = reqwest::Client::new();
    let token = env::var("GIPHY_TOKEN").expect("Expected giphy.com API in environment");

    // By default it will fetch 50 trending stickers
    let amount = 50;
    let url = format!("https://api.giphy.com/v1/stickers/trending?api_key={}&limit={}", token, amount);
    
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

#[poise::command(slash_command, description_localized("en-US", "Fetch a gif from giphy.com"))]
async fn gif(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching a gif from giphy.com").await.unwrap();

    let client = reqwest::Client::new();
    let token = env::var("GIPHY_TOKEN").expect("Expected giphy.com API in environment");

    // By default it will fetch 50 trending stickers
    let amount = 50;
    let url = format!("https://api.giphy.com/v1/gifs/trending?api_key={}&limit={}", token, amount);
    
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

#[poise::command(slash_command, description_localized("en-US", "Fetch random image from pexels.com"))]
async fn image(
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

    let random_index = rand::random::<usize>() % 50;
    let image_url = Url::parse(parsed_json["photos"][random_index]["src"]["original"].as_str().unwrap()).unwrap();
    let image_bytes = client.get(image_url.clone()).send().await.unwrap().bytes().await.unwrap();
    std::fs::write("./image.jpeg", image_bytes).unwrap();

    let picture = CreateMessage::new()
    .content("Here's a random picture from pexels.com")
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
async fn quote(
    ctx: Context<'_>,
    topic: QuoteTopic,
) -> Result<(), Error> {
    let fetching = ctx.say("Fetching a quote from apininjas.com").await.unwrap();

    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");

    let client = reqwest::Client::new();
    let url = format!("https://api.api-ninjas.com/v1/quotes?category={}", topic.name().to_lowercase());
    println!("{url}");
    
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

#[poise::command(slash_command, description_localized("en-US", "Throw a joke"))]
async fn joke(
    ctx: Context<'_>
) -> Result<(), Error> {
    let fetching = ctx.say("Thinking of a joke...").await.unwrap();

    let mut joker_instance: Joker = Joker::new();

    // Chainable API
    joker_instance
        .add_categories(&mut vec![Category::Programming, Category::Pun, Category::Dark, Category::Spooky]);
    
    // get JSON joke
    let joke = joker_instance.get_joke().unwrap();

    let content: String;

    if joke["type"].as_str().unwrap() == "twopart" {
        let setup = joke["setup"].as_str().unwrap();
        let delivery = joke["delivery"].as_str().unwrap();
        content = format!("{}\n{}", setup, delivery);

    } else {
        content = format!("{}", joke["joke"].as_str().unwrap());
    }
    
    let edited_message = CreateReply {
        content: Some(content),
        ..Default::default()
    };

    if let Err(why) = fetching.edit(ctx, edited_message).await {
        println!("Error sending message: {why:?}");
    }

    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Send a cute cat photo ><"))]
async fn cat(
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

#[poise::command(prefix_command)]
pub async fn register(
    ctx: Context<'_>
) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: vec![register(), ping(), advice(), sticker(), gif(), image(), quote(), joke(), cat()],
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            println!("{} connected!", _ready.user.name);
            poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
            Ok(Data {})
        })
    })
    .build();

    // Build our client.
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        // .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}