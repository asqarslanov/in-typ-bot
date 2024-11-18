use std::fmt::Display;

use indoc::formatdoc;

use crate::Locale;

pub(crate) mod _help;

pub struct Section {
    pub(crate) _locale: Locale,
}

impl Section {
    pub fn start<S: Display>(&self, snippet: S) -> String {
        match self._locale {
            Locale::EnUs => formatdoc! {
                "
                    Hello! I am {}!
                    Send me some Typst code, and I will render it here.

                    For example:
                    {snippet}

                    See {} for more details.\
                ",
                crate::shared::bot::USERNAME,
                crate::shared::commands::HELP
            },
            Locale::RuRu => formatdoc! {
                "
                    Привет! Я {}!
                    Отправьте мне код Typst, и я отрисую его здесь.

                    Например:
                    {snippet}

                    Смотрите {} для подробностей.\
                ",
                crate::shared::bot::USERNAME,
                crate::shared::commands::HELP
            },
        }
    }

    pub const fn help(&self) -> _help::FormatInlineSnippet {
        _help::FormatInlineSnippet {
            _locale: self._locale,
        }
    }
}
