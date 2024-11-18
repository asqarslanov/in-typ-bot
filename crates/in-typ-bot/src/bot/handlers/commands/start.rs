use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::html;

use crate::bot::common::BotLocale;

pub async fn handle(bot: Bot, msg: Message) {
    let locale = BotLocale::from(&msg);
    let t = i18n::locale(locale.into()).commands;

    let text = t
        .start()
        .format_snippet(|it| html::code_block_with_lang(it, "typst"));

    let _ = bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}
