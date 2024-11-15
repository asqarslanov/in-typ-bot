use std::iter;

use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedPhoto, InputFile,
    InputMessageContent, InputMessageContentText, ParseMode,
};
use teloxide::utils::markdown;
use tokio::fs;
use uuid::Uuid;

use crate::logic::{self, Quality};

pub async fn process_message(bot: Bot, msg: Message, cache_chat: ChatId) -> anyhow::Result<()> {
    let contents = msg.text().expect("message contains no text");

    match logic::render(contents, Quality::High).await? {
        Ok(path) => {
            let photo = InputFile::file(&path);

            let cached_msg = bot.send_photo(cache_chat, photo).await?;
            bot.copy_message(msg.chat.id, cache_chat, cached_msg.id)
                .await?;

            fs::remove_file(path).await?;
            bot.delete_message(cache_chat, cached_msg.id).await?;
        }
        Err(ref err_msg) => {
            let text = markdown::code_block_with_lang(err_msg, "stderr");
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
    }

    Ok(())
}

pub async fn process_inline(
    bot: Bot,
    query: InlineQuery,
    cache_chat: ChatId,
) -> anyhow::Result<()> {
    let contents = query.query;

    match logic::render(&contents, Quality::Low).await? {
        Ok(path) => {
            let photo = InputFile::file(&path);
            let cached_msg = bot.send_photo(cache_chat, photo).await?;

            bot.answer_inline_query(
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
            .await?;

            fs::remove_file(path).await?;
            bot.delete_message(cache_chat, cached_msg.id).await?;
        }
        Err(ref err_msg) => {
            let text = markdown::code_block_with_lang(err_msg, "stderr");
            bot.answer_inline_query(
                query.id,
                iter::once(InlineQueryResult::Article(InlineQueryResultArticle::new(
                    Uuid::new_v4().simple().to_string(),
                    "yo",
                    InputMessageContent::Text(
                        InputMessageContentText::new(text).parse_mode(ParseMode::MarkdownV2),
                    ),
                ))),
            )
            .send()
            .await?;
        }
    }

    Ok(())
}
