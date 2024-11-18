use indoc::indoc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::html;

pub async fn handle(bot: Bot, msg: Message) {
    let snippet = html::code_block_with_lang(
        indoc! {"
            === Euler's identity:
            #let exponent = $i pi$
            $e^exponent + 1 = 0$\
        "},
        "typst",
    );

    let text = i18n::locale(i18n::Locale::default())
        .commands
        .start(snippet);

    let _ = bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}
