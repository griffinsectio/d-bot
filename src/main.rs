mod commands;
use commands::ping::ping;
use commands::advice::advice;
use commands::cat::cat;
use commands::gif::gif;
use commands::image::image;
use commands::joke::joke;
use commands::music::music;
use commands::quote::quote;
use commands::sticker::sticker;
use commands::trivia::trivia;
use commands::about::about;

use std::env;
use std::process;
use std::fs;

use serde_json::{self, Value};

use reqwest::Url;

use serenity::all::CreateMessage;
use serenity::builder::CreateAttachment;

use poise::{serenity_prelude as serenity, ChoiceParameter, CreateReply};

use regex::Regex;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
        commands: vec![register(), ping(), advice(), sticker(), gif(), image(), quote(), joke(), cat(), music(), trivia(), about()],
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