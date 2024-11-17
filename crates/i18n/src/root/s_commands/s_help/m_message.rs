use std::fmt::Display;

use indoc::formatdoc;

use crate::Locale;

pub struct Edit<T: Display> {
    pub(crate) _locale: Locale,
    pub(crate) inline_snippet: T,
}

impl<T: Display> Edit<T> {
    pub fn edit_clarification(&self, f: impl FnOnce(&'static str) -> String) -> String {
        match self._locale {
            Locale::EnUs => {
                formatdoc!(
                    "
                        I’m a bot that can render Typst markup in Telegram chats.

                        If you’re not familiar with Typst syntax, refer to their official documentation: typst.app/docs/.

                        To use me in inline mode, type the following inside any chat:
                        {}
                        {}

                        Or you can just use me in chat mode by sending me messages directly.

                        Author: @AsqArslanov
                        Source code: github.com/asqarslanov/in-typ-bot\
                    ",
                    self.inline_snippet,
                    f("…of course, you can write any other Typst code."),
                )
            }
            Locale::RuRu => todo!(),
        }
    }
}
