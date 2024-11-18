use std::fmt::Display;

use indoc::formatdoc;

use crate::Locale;

pub struct AddInlineSnippet {
    pub(crate) _locale: Locale,
}

impl AddInlineSnippet {
    pub fn with_inline_snippet<S: Display>(self, inline_snippet: S) -> FormatClarification<S> {
        FormatClarification {
            _locale: self._locale,
            inline_snippet,
        }
    }
}

pub struct FormatClarification<S1>
where
    S1: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) inline_snippet: S1,
}

impl<S1> FormatClarification<S1>
where
    S1: Display,
{
    pub fn format_clarification<S: Display>(&self, f: impl FnOnce(&'static str) -> S) -> String {
        match self._locale {
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
