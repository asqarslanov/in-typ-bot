use indoc::{formatdoc, indoc};
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::markdown;

pub async fn start(bot: Bot, msg: Message) {
    let snippet = markdown::code_block_with_lang(
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
        .parse_mode(ParseMode::MarkdownV2)
        .send_message(msg.chat.id, text)
        .await;
}

pub async fn help(bot: Bot, msg: Message) {
    let _ = bot
        .parse_mode(ParseMode::MarkdownV2)
        .send_message(msg.chat.id, "A help message should go here…")
        // .parse_mode(ParseMode::MarkdownV2)
        .await;
}
