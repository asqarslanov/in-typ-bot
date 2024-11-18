use teloxide::prelude::*;
use teloxide::types::{LinkPreviewOptions, ParseMode};
use teloxide::utils::html;

use crate::bot::common::BotLocale;

pub async fn handle(bot: Bot, msg: Message) {
    let locale = BotLocale::from(&msg);
    let t = i18n::t(locale.into()).commands;

    let text = t
        .help()
        .format_inline_snippet(html::code_inline)
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
