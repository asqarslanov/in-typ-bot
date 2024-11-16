use std::iter;

use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedPhoto, InputFile,
    InputMessageContent, InputMessageContentText, ParseMode,
};
use teloxide::utils::markdown;
use tokio::fs;
use uuid::Uuid;

use crate::logic::{self};

pub async fn process_message(bot: Bot, msg: Message, cache_chat: ChatId) -> Result<(), ()> {
    let contents = msg.text().expect("message contains no text");

    match logic::render(contents).await {
        Ok(path) => {
            let photo = InputFile::file(&path);

            let cached_msg = bot
                .send_photo(cache_chat, photo)
                .await
                .expect("cached message should successfully be sent");
            let _ = bot
                .copy_message(msg.chat.id, cache_chat, cached_msg.id)
                .await;

            let _ = fs::remove_file(path).await;
            let _ = bot.delete_message(cache_chat, cached_msg.id).await;
        }
        Err(err) => match err {
            logic::RenderError::Io(_) => todo!(),
            logic::RenderError::EmptyDocument => return Ok(()),
            logic::RenderError::InvalidSyntax {
                coordinates,
                message,
            } => {
                let text = generate_error_text(contents, coordinates, &message, true);
                let _ = bot
                    .send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await;
            }
        },
    }

    Ok(())
}

fn generate_error_text(
    source: &str,
    coordinates: (u32, u32),
    message: &str,
    formatting: bool,
) -> String {
    let (line, column) = coordinates;

    if formatting {
        let source = markdown::code_block_with_lang(source, "typst");
        let coordinates_text = markdown::bold(&format!("Error on Line {line} : Column {column}"));
        let error_text = markdown::code_inline(message);
        format!("{source}\n{coordinates_text}\n{error_text}")
    } else {
        format!("{line}:{column}: {message}")
    }
}

pub async fn process_inline(bot: Bot, query: InlineQuery, cache_chat: ChatId) -> Result<(), ()> {
    let contents = query.query;

    match logic::render(&contents).await {
        Ok(path) => {
            let photo = InputFile::file(&path);
            let cached_msg = bot
                .send_photo(cache_chat, photo)
                .await
                .expect("cached message should be sent successfully 2");

            let _ = bot
                .answer_inline_query(
                    query.id,
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

            let _ = fs::remove_file(path).await;
            let _ = bot.delete_message(cache_chat, cached_msg.id).await;
        }
        Err(err) => match err {
            logic::RenderError::Io(_) => todo!(),
            logic::RenderError::EmptyDocument => return Ok(()),
            logic::RenderError::InvalidSyntax {
                coordinates,
                message,
            } => {
                let not_formatted = generate_error_text(&contents, coordinates, &message, false);
                let formatted = generate_error_text(&contents, coordinates, &message, true);

                let _ = bot
                    .answer_inline_query(
                        query.id,
                        iter::once(InlineQueryResult::Article(InlineQueryResultArticle::new(
                            Uuid::new_v4().simple().to_string(),
                            not_formatted,
                            InputMessageContent::Text(
                                InputMessageContentText::new(formatted)
                                    .parse_mode(ParseMode::MarkdownV2),
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
