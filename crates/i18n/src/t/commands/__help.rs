use std::fmt::Display;

use const_format::formatcp;
use indoc::formatdoc;

use crate::Locale;

pub struct FormatInlineSnippet {
    pub(crate) __locale: Locale,
}

impl FormatInlineSnippet {
    /// ```text
    /// @InTypeBot $2 + 2 = 5$
    /// ```
    pub fn format_inline_snippet<S: Display>(
        &self,
        f: impl FnOnce(&str) -> S,
    ) -> FormatClarification<S> {
        FormatClarification {
            __locale: self.__locale,
            inline_snippet: f(match self.__locale {
                Locale::EnUs => formatcp!("{} $2 + 2 = 5$", crate::shared::bot::USERNAME),
                Locale::RuRu => formatcp!("{} $2 + 2 = 5$", crate::shared::bot::USERNAME),
            }),
        }
    }
}

pub struct FormatClarification<S1>
where
    S1: Display,
{
    pub(crate) __locale: Locale,
    pub(crate) inline_snippet: S1,
}

impl<S1> FormatClarification<S1>
where
    S1: Display,
{
    /// ```text
    /// …of course, you can write any other Typst code.
    /// ```
    pub fn format_clarification<S: Display>(&self, f: impl FnOnce(&str) -> S) -> String {
        match self.__locale {
            Locale::EnUs => formatdoc!(
                "
                    I’m a bot that can render Typst markup in Telegram chats.

                    If you’re not familiar with Typst syntax, refer to their official documentation: {}.

                    To use me in inline mode, type the following inside any chat:
                    {}
                    {}

                    Or you can just use me in chat mode by sending me messages directly.

                    Author: {}
                    Source code: {}\
                ",
                crate::shared::TYPST_DOCS,
                self.inline_snippet,
                f("…of course, you can write any other Typst code."),
                crate::shared::bot::AUTHOR,
                crate::shared::bot::SOURCE_CODE,
            ),
            Locale::RuRu => formatdoc!(
                "
                    Я бот, который умеет отрисовывать разметку Typst в Telegram-чатах.

                    Если вы не знакомы с синтаксисом Typst, ознакомьтесь с их официальной документацией: {}.

                    Чтобы использовать меня в inline-режиме, введите подобный текст внутри любого чата:
                    {}
                    {}

                    Или же вы можете просто использовать меня в режиме чата, отправляя сообщения напрямую.

                    Автор: {}
                    Исходный код: {}\
                ",
                crate::shared::TYPST_DOCS,
                self.inline_snippet,
                f("…разумеется, вы можете использовать любой другой Typst-код."),
                crate::shared::bot::AUTHOR,
                crate::shared::bot::SOURCE_CODE,
            ),
        }
    }
}
