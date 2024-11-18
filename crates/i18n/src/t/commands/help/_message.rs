use std::fmt::Display;

use indoc::formatdoc;

use crate::Locale;

pub struct Format<S1, S2, S3, S4>
where
    S1: Display,
    S2: Display,
    S3: Display,
    S4: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) typst_documentation: S1,
    pub(crate) inline_snippet: S2,
    pub(crate) author: S3,
    pub(crate) source_code: S4,
}

impl<S1, S2, S3, S4> Format<S1, S2, S3, S4>
where
    S1: Display,
    S2: Display,
    S3: Display,
    S4: Display,
{
    pub fn format_clarification<S: Display>(&self, f: impl FnOnce(&'static str) -> S) -> String {
        match self._locale {
            Locale::EnUs => {
                formatdoc!(
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
                    self.typst_documentation,
                    self.inline_snippet,
                    f("…of course, you can write any other Typst code."),
                    self.author,
                    self.source_code,
                )
            }
            Locale::RuRu => todo!(),
        }
    }
}
