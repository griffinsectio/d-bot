use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Trivia video games questions game"))]
pub async fn trivia(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "https://opentdb.com/api.php?amount=10&category=15";
    let response = client.get(url).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    let parsed_json: Value = serde_json::from_str(response_text.as_str()).unwrap();
    
    if parsed_json["response_code"].as_i64().unwrap() == 0 {
        let uuid_question = ctx.id();
        let questions = parsed_json["results"].as_array().unwrap();

        let mut correct_answer_user = 0;

        for question in questions {
            let mut choices = vec![];
            let question_text = html_escape::decode_html_entities(question["question"].as_str().unwrap()).to_string();
            let correct_answer = html_escape::decode_html_entities(question["correct_answer"].as_str().unwrap()).to_string();
            let incorrect_answers = question["incorrect_answers"].as_array().unwrap();

            println!("Question: {}", question_text);
            let correct_answer_index: usize;
            println!("{}", &correct_answer);

            if question["type"].as_str().unwrap() == "multiple" {
                correct_answer_index = rand::random::<usize>() % 4;
            } else {
                correct_answer_index = rand::random::<usize>() % 2;
            }

            for incorrect_answer in incorrect_answers {
                choices.push(html_escape::decode_html_entities(incorrect_answer.as_str().unwrap()).to_string());
            }

            choices.insert(correct_answer_index, correct_answer);

            let question =
            {
                let mut components = vec![];

                if question["type"].as_str().unwrap() == "multiple" {
                    components.push(
                        serenity::CreateActionRow::Buttons(vec![
                            serenity::CreateButton::new(format!("{}", uuid_question + 0))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[0])),
                            serenity::CreateButton::new(format!("{}", uuid_question + 1))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[1])),
                            serenity::CreateButton::new(format!("{}", uuid_question + 2))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[2])),
                            serenity::CreateButton::new(format!("{}", uuid_question + 3))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[3])),
                        ])
                    );
                } else {
                    components.push(
                        serenity::CreateActionRow::Buttons(vec![
                            serenity::CreateButton::new(format!("{}", uuid_question + 0))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[0])),
                            serenity::CreateButton::new(format!("{}", uuid_question + 1))
                                .style(serenity::ButtonStyle::Primary)
                                .label(format!("{}", choices[1])),
                        ])
                    );
                }                    
        
                CreateMessage::new()
                .content(format!("{question_text}"))
                .components(components)
            };

            ctx.channel_id().send_message(&ctx.http(), question).await.unwrap();

            while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(120))
            .filter(
                move |mci| 
                mci.data.custom_id == (uuid_question + 0).to_string() || 
                mci.data.custom_id == (uuid_question + 1).to_string() ||
                mci.data.custom_id == (uuid_question + 2).to_string() || 
                mci.data.custom_id == (uuid_question + 3).to_string()
            )
            .await
            {

                let mut msg = mci.message.clone();
                if mci.data.custom_id.trim().parse::<usize>().unwrap() == uuid_question as usize + correct_answer_index {
                    msg.edit(
                        ctx,
                        serenity::EditMessage::new().content(format!("That's right! :D")),
                    )
                    .await.unwrap();
                    
                    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                    .await.unwrap();

                    correct_answer_user += 1;

                    std::thread::sleep(std::time::Duration::from_millis(1500));
                    msg.delete(&ctx.http()).await.unwrap();

                    break;
                } else {
                    msg.edit(
                        ctx,
                        serenity::EditMessage::new().content(format!("That's incorrect :(")),
                    )
                    .await.unwrap();
                    
                    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                    .await.unwrap();

                    std::thread::sleep(std::time::Duration::from_millis(1500));
                    msg.delete(&ctx.http()).await.unwrap();

                    break;
                }
            }
        }

        let result = format!("You guessed {} right answer out of {} question", correct_answer_user, questions.len());
        ctx.channel_id().say(&ctx.http(), result).await.unwrap();

        if correct_answer_user == 0 {
            ctx.channel_id().say(&ctx.http(), "It's okay, you doesn't have to know everything :)").await.unwrap();
        } else if correct_answer_user == 10 {
            ctx.channel_id().say(&ctx.http(), "You're really good at this huh? XD").await.unwrap();
        } else if correct_answer_user > 0 {
            ctx.channel_id().say(&ctx.http(), "Ay, Not bad ;)").await.unwrap();
        } else if correct_answer_user > 5 {
            ctx.channel_id().say(&ctx.http(), "That's awesome :D").await.unwrap();
        } 
    }

    Ok(())
}