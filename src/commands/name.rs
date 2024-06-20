
use crate::*;

#[derive(Debug, poise::ChoiceParameter)]
enum Gender {
    #[name = "Male"]
    Male,
    #[name = "Female"]
    Female,
}

#[poise::command(slash_command, description_localized("en-US", "Generate a random male/female name"))]
pub async fn name(
    ctx: Context<'_>,
    gender: Gender,
) -> Result<(), Error> {
    let thinking = ctx.say("Thinking of a name...").await.unwrap();

    let url = match gender {
        Gender::Male => "https://api.api-ninjas.com/v1/babynames?gender=boy",
        Gender::Female => "https://api.api-ninjas.com/v1/babynames?gender=girl"
    };

    let client = reqwest::Client::new();
    let token = env::var("API_NINJA_TOKEN").expect("Expected a token in the environment");
    let response = client.get(url).header("X-Api-Key", token).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let result: Value = serde_json::from_str(response_text.as_str()).unwrap();
    let name_arr = result.as_array().unwrap();

    let random_name_index = rand::random::<usize>() % name_arr.len();

    let name = CreateReply {
        content: Some(name_arr[random_name_index].as_str().unwrap().to_string()),
        ..Default::default()
    };

    if let Err(why) = thinking.edit(ctx, name).await {
        println!("Error sending message: {}", why);
    }

    Ok(())
}