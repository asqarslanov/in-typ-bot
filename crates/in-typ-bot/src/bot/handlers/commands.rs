use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::RequestError;

mod help;
mod start;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum Command {
    Start,
    Help,
}

pub async fn handle(bot: Bot, message: Message, command: Command) -> Result<(), RequestError> {
    match command {
        Command::Start => {
            start::handle(bot, message).await;
            Ok(())
        }
        Command::Help => {
            help::handle(bot, message).await;
            Ok(())
        }
    }
}
