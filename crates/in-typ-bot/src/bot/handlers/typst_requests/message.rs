use teloxide::RequestError;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use tokio::fs;

use crate::bot::common::BotLocale;
use crate::logic;

pub async fn handle(bot: Bot, message: Message) -> Result<(), RequestError> {
    let Some(contents) = message.text() else {
        return Ok(());
    };

    let t = i18n::t(BotLocale::from(&message).into()).service;

    let reply_msg = bot
        .send_message(message.chat.id, t.wait())
        .parse_mode(ParseMode::Html)
        .disable_notification(true)
        .await?;

    match logic::render(contents).await {
        Ok(path) => {
            let photo = InputFile::file(&path);

            let _ = bot
                .edit_message_media(
                    reply_msg.chat.id,
                    reply_msg.id,
                    InputMedia::Photo(InputMediaPhoto::new(photo)),
                )
                .await;

            let _ = fs::remove_file(path).await;
        }
        Err(err) => match err {
            logic::RenderError::Io(_) => todo!(),
            logic::RenderError::EmptyDocument => return Ok(()),

            logic::RenderError::Logic(ref error) => {
                let text = super::generate_error_text(contents, error, true);
                let _ = bot
                    .send_message(message.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .await;
            }
        },
    }

    Ok(())
}
