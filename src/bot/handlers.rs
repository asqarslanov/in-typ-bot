use std::iter;

use indoc::formatdoc;
use itertools::Itertools;
use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedPhoto, InputFile,
    InputMedia, InputMediaPhoto, InputMessageContent, InputMessageContentText, ParseMode,
};
use teloxide::utils::html;
use teloxide::RequestError;
use tokio::fs;
use uuid::Uuid;

use super::Command;
use crate::logic::{self, ErrorDetails};

mod commands;

pub async fn command(bot: Bot, msg: Message, cmd: Command) -> Result<(), RequestError> {
    match cmd {
        Command::Start => {
            commands::start(bot, msg).await;
            Ok(())
        }
        Command::Help => {
            commands::help(bot, msg).await;
            Ok(())
        }
    }
}

pub async fn message(bot: Bot, msg: Message) -> Result<(), RequestError> {
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
                let text = generate_error_text(contents, errors, true);
                let _ = bot
                    .send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .await;
            }
        },
    }

    Ok(())
}

fn generate_error_text(source_code: &str, errors: &[ErrorDetails], formatting: bool) -> String {
    if formatting {
        let errors_formatted = errors
            .iter()
            .map(|err| {
                let (line, column) = err.coordinates;
                let coordinates_text =
                    html::bold(&format!("Error on Line {line} : Column {column}"));
                let error_text = html::code_inline(&err.message);
                formatdoc! {"
                    {coordinates_text}
                    {error_text}\
                "}
            })
            .join("\n\n");

        let source_code_formatted = html::code_block_with_lang(source_code, "typst");
        formatdoc! {"
            {source_code_formatted}

            {errors_formatted}\
        "}
    } else {
        let error = errors
            .first()
            .expect("typst should output at least one error");

        format!(
            "{}:{}: {}",
            error.coordinates.0, error.coordinates.1, error.message,
        )
    }
}

pub async fn inline_query(
    bot: Bot,
    qry: InlineQuery,
    cache_chat: ChatId,
) -> Result<(), RequestError> {
    let contents = qry.query;

    match logic::render(&contents).await {
        Ok(path) => {
            let photo = InputFile::file(&path);

            let cached_msg = bot.send_photo(cache_chat, photo).await?;
            let _ = fs::remove_file(path).await;

            let _ = bot
                .answer_inline_query(
                    qry.id,
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
            logic::RenderError::InvalidSyntax(ref errors) => {
                let not_formatted = generate_error_text(&contents, errors, false);
                let formatted = generate_error_text(&contents, errors, true);

                let _ = bot
                    .answer_inline_query(
                        qry.id,
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
