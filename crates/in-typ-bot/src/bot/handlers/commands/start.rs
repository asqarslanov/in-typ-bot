use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::html;

use crate::bot::common::BotLocale;

pub async fn handle(bot: Bot, message: Message) {
    let locale = BotLocale::from(&message);
    let t = i18n::t(locale.into()).commands;

    let text = t
        .start()
        .format_snippet(|it| html::code_block_with_lang(it, "typst"));

    let _ = bot
        .send_message(message.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}
