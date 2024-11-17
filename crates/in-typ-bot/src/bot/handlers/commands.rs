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

pub async fn handle(bot: Bot, msg: Message, cmd: Command) -> Result<(), RequestError> {
    match cmd {
        Command::Start => {
            start::handle(bot, msg).await;
            Ok(())
        }
        Command::Help => {
            help::handle(bot, msg).await;
            Ok(())
        }
    }
}
