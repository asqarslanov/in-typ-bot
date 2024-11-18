use indoc::indoc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::html;

use crate::bot::common::BotLocale;

pub async fn handle(bot: Bot, msg: Message) {
    let locale = BotLocale::from(&msg);
    let t = i18n::locale(locale.into()).commands;

    let snippet = html::code_block_with_lang(
        indoc! {"
            === Euler's identity:
            #let exponent = $i pi$
            $e^exponent + 1 = 0$\
        "},
        "typst",
    );

    let text = t.start(snippet);

    let _ = bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}
