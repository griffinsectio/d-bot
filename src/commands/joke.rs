use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Throw a joke"))]
pub async fn joke(
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