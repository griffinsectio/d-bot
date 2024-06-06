use chrono::Local;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    let time = Local::now().format("%H:%M WIB");
    format!("The time now is: {}", time)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("time").description("Show current time in Indonesia's WIB")
}
