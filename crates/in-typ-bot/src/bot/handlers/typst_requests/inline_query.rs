use std::iter;

use teloxide::RequestError;
use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedPhoto, InputFile,
    InputMessageContent, InputMessageContentText, ParseMode,
};
use tokio::fs;
use uuid::Uuid;

use crate::logic;

pub async fn handle(
    bot: Bot,
    inline_query: InlineQuery,
    cache_chat: ChatId,
) -> Result<(), RequestError> {
    let contents = inline_query.query;

    match logic::render(&contents).await {
        Ok(path) => {
            let photo = InputFile::file(&path);

            let cached_msg = bot.send_photo(cache_chat, photo).await?;
            let _ = fs::remove_file(path).await;

            let _ = bot
                .answer_inline_query(
                    inline_query.id,
                    iter::once(InlineQueryResult::CachedPhoto(
                        InlineQueryResultCachedPhoto::new(
                            Uuid::new_v4().simple().to_string(),
                            &cached_msg
                                .photo()
                                .expect("cached message should contain photos")
                                .first()
                                .expect("cached message should contain at least one photo")
                                .file
                                .id,
                        ),
                    )),
                )
                .send()
                .await;

            let _ = bot.delete_message(cache_chat, cached_msg.id).await;
        }
        Err(err) => match err {
            logic::RenderError::Io(_) => todo!(),
            logic::RenderError::EmptyDocument => return Ok(()),
            logic::RenderError::Logic(ref errors) => {
                let not_formatted = super::generate_error_text(&contents, errors, false);
                let formatted = super::generate_error_text(&contents, errors, true);

                let _ = bot
                    .answer_inline_query(
                        inline_query.id,
                        iter::once(InlineQueryResult::Article(InlineQueryResultArticle::new(
                            Uuid::new_v4().simple().to_string(),
                            not_formatted,
                            InputMessageContent::Text(
                                InputMessageContentText::new(formatted).parse_mode(ParseMode::Html),
                            ),
                        ))),
                    )
                    .send()
                    .await;
            }
        },
    }

    Ok(())
}
