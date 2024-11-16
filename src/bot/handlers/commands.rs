use indoc::{formatdoc, indoc};
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::html;

pub async fn start(bot: Bot, msg: Message) {
    let snippet = html::code_block_with_lang(
        indoc! {"
            === Euler's identity:
            #let exponent = $i pi$
            $e^exponent + 1 - 0$\
        "},
        "typst",
    );
    let text = formatdoc! {"
        Hello! I am @InTypBot!
        Send me some Typst code and I will render it here.

        For example:
        {snippet}

        See /help for more details.\
    "};

    let _ = bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}

pub async fn help(bot: Bot, msg: Message) {
    let _ = bot
        .send_message(msg.chat.id, "A help message should go here…")
        .parse_mode(ParseMode::Html)
        // .parse_mode(ParseMode::Html)
        .await;
}
