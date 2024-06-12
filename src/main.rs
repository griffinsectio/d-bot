mod commands;

use std::{env, result};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

use joketeller::{
    Joker, Category,
};

use reqwest::Url;

use serenity::all::CreateMessage;
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateAttachment, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::channel::Message;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::Timestamp;
use serenity::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
// struct Quote {
//     quote_map: HashMap<String, String>
// }

struct Handler;

// #[async_trait]
// impl EventHandler for Handler {
//     // Set a handler for the `message` event. This is called whenever a new message is received.
//     //
//     // Event handlers are dispatched through a threadpool, and so multiple events can be
//     // dispatched simultaneously.
//     async fn message(&self, ctx: Context, msg: Message) {
//         if msg.content == "!ping" {
//             // Sending a message can fail, due to a network error, an authentication error, or lack
//             // of permissions to post in the channel, so log to stdout when some error happens,
//             // with a description of it.
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
//                 println!("Error sending message: {why:?}");
//             }
//         } else if msg.content == "!hello" {
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Hi there!").await {
//                 println!("Error sending message: {why:?}");
//             }
//         }
//     }

//     // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
//     // a READY payload is sent by Discord. This payload contains data like the current user's guild
//     // Ids, current user data, private channels, and more.
//     //
//     // In this case, just print what the current user's username is.
//     async fn ready(&self, _: Context, ready: Ready) {
//         println!("{} is connected!", ready.user.name);
//     }
// }

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                "ping" => Some(commands::ping::run(&command.data.options())),
                "date" => Some(commands::date::run(&command.data.options())),
                "time" => Some(commands::time::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!quote" {
            let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");

            let topics = vec!["learning", "intelligence", "knowledge", "leadership", "success"];
            let rand_index = rand::random::<usize>() % topics.len();
            let rand_topic = topics[rand_index];

            let client = reqwest::Client::new();
            let url = format!("https://api.api-ninjas.com/v1/quotes?category={}", rand_topic);
            
            let response = client.get(url).header("X-Api-Key", token).send().await.unwrap();

            let body = response.text().await.unwrap();

            let result: Value = serde_json::from_str(body.as_str()).unwrap();

            let quote = format!("\"{}\"", result[0]["quote"]);
            let author = format!("\\- {}", result[0]["author"]);

            let content = format!("{}\n{}", quote, author);

            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!joke" {
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
            
            if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!cat" {
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

            let msg = msg.channel_id.send_message(&ctx.http, picture).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            } else {
                std::fs::remove_file("./cat.jpg").unwrap();
            }
        } else if msg.content == "!advice" {
            let client = reqwest::Client::new();
            let url = "https://api.adviceslip.com/advice";

            let response = client.get(url).send().await.unwrap();

            let response_json = response.text().await.unwrap();

            let result_json: serde_json::Value = serde_json::from_str(&response_json.as_str()).unwrap();

            let content = CreateMessage::new().content(result_json["slip"]["advice"].as_str().unwrap());

            if let Err(why) = msg.channel_id.send_message(&ctx.http, content).await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!image" {
            let topics = vec!["cat", "dog", "nature", "computer", "ai", "painting"];
            let random_topic = topics[rand::random::<usize>() % topics.len()];
            let pictures_amount = 50;

            let client = reqwest::Client::new();
            let url = format!("https://api.pexels.com/v1/search?query={}&per_page={}", random_topic, pictures_amount);
            let token = env::var("PEXELS_TOKEN").expect("Expected pexels.com token");
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

            let msg = msg.channel_id.send_message(&ctx.http, picture).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            } else {
                std::fs::remove_file("./image.jpeg").unwrap();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::date::register(),
                commands::time::register(),
                commands::id::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register()).await;
        println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
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