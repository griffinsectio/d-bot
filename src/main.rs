mod commands;

use std::env;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

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
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                "modal" => {
                    commands::modal::run(&ctx, &command).await.unwrap();
                    None
                },
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
                commands::id::register(),
                commands::welcome::register(),
                commands::numberinput::register(),
                commands::attachmentinput::register(),
                commands::modal::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
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