use teloxide::prelude::*;
// use teloxide::types::ParseMode;

pub async fn start(bot: Bot, msg: Message) {
    let _ = bot
        .send_message(msg.chat.id, "A greeting message should go here…")
        // .parse_mode(ParseMode::MarkdownV2)
        // .disable_notification(true)
        .await;
}

pub async fn help(bot: Bot, msg: Message) {
    let _ = bot
        .send_message(msg.chat.id, "A help message should go here…")
        // .parse_mode(ParseMode::MarkdownV2)
        // .disable_notification(true)
        .await;
}
