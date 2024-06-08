use chrono::Local;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    let date = Local::now().format("%a, %d-%b-%Y").to_string();
    format!("The date now is: {}\nNote: The time is based on western Indonesian time", date)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("date").description("Show current date in western Indonesian time")
}
