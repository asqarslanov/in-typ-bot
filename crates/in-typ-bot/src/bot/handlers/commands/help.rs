use i18n::Locale;
use teloxide::prelude::*;
use teloxide::types::{LinkPreviewOptions, ParseMode};
use teloxide::utils::html;

pub async fn handle(bot: Bot, msg: Message) {
    let t = i18n::locale(Locale::EnUs).commands.help;

    let text = t
        .message(
            "typst.app/docs/",
            html::code_inline("@InTypBot $2 + 2 = 5$"),
            "@AsqArslanov",
            "github.com/asqarslanov/in-typ-bot",
        )
        .format_clarification(html::italic);

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
