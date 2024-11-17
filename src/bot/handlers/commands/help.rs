use indoc::formatdoc;
use teloxide::prelude::*;
use teloxide::types::{LinkPreviewOptions, ParseMode};
use teloxide::utils::html;

pub async fn handle(bot: Bot, msg: Message) {
    let inline_snippet = html::code_inline("@InTypBot $2 + 2 = 5$");
    let clarification = html::italic("…of course, you can write any other Typst code.");

    let text = formatdoc! {"
        I’m a bot that can render Typst markup in Telegram chats.

        If you’re not familiar with Typst syntax, refer to their official documentation: typst.app/docs/.

        To use me in inline mode, type the following inside any chat:
        {inline_snippet}
        {clarification}

        Or you can just use me in chat mode by sending me messages directly.

        Source code: github.com/asqarslanov/in-typ-bot
        Author: @AsqArslanov\
    "};

    let _ = bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .link_preview_options(LinkPreviewOptions {
            is_disabled: true,
            prefer_large_media: false,
            prefer_small_media: false,
            show_above_text: false,
            url: None,
        })
        .await;
}
