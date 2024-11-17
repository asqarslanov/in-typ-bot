use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use teloxide::RequestError;
use tokio::fs;

use crate::logic;

pub async fn handle(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let Some(contents) = msg.text() else {
        return Ok(());
    };

    let reply_msg = bot
        .send_message(msg.chat.id, "Wait a second…")
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
            logic::RenderError::InvalidSyntax(ref errors) => {
                let text = super::generate_error_text(contents, errors, true);
                let _ = bot
                    .send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .await;
            }
        },
    }

    Ok(())
}
