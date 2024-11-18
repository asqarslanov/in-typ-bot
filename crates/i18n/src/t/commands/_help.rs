use std::fmt::Display;

use indoc::formatdoc;

use crate::Locale;

pub struct AddTypstDocs {
    pub(crate) _locale: Locale,
}

impl AddTypstDocs {
    pub const fn with_typst_docs<S: Display>(self, typst_docs: S) -> AddInlineSnippet<S> {
        AddInlineSnippet {
            _locale: self._locale,
            typst_docs,
        }
    }
}

pub struct AddInlineSnippet<S1>
where
    S1: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) typst_docs: S1,
}

impl<S1: Display> AddInlineSnippet<S1> {
    pub fn with_inline_snippet<S: Display>(self, inline_snippet: S) -> AddAuthor<S1, S> {
        AddAuthor {
            _locale: self._locale,
            typst_docs: self.typst_docs,
            inline_snippet,
        }
    }
}

pub struct AddAuthor<S1, S2>
where
    S1: Display,
    S2: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) typst_docs: S1,
    pub(crate) inline_snippet: S2,
}

impl<S1, S2> AddAuthor<S1, S2>
where
    S1: Display,
    S2: Display,
{
    pub fn with_author<S: Display>(self, author: S) -> AddSourceCode<S1, S2, S> {
        AddSourceCode {
            _locale: self._locale,
            typst_docs: self.typst_docs,
            inline_snippet: self.inline_snippet,
            author,
        }
    }
}

pub struct AddSourceCode<S1, S2, S3>
where
    S1: Display,
    S2: Display,
    S3: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) typst_docs: S1,
    pub(crate) inline_snippet: S2,
    pub(crate) author: S3,
}

impl<S1, S2, S3> AddSourceCode<S1, S2, S3>
where
    S1: Display,
    S2: Display,
    S3: Display,
{
    pub fn with_source_code<S: Display>(
        self,
        source_code: S,
    ) -> FormatClarification<S1, S2, S3, S> {
        FormatClarification {
            _locale: self._locale,
            typst_docs: self.typst_docs,
            inline_snippet: self.inline_snippet,
            author: self.author,
            source_code,
        }
    }
}

pub struct FormatClarification<S1, S2, S3, S4>
where
    S1: Display,
    S2: Display,
    S3: Display,
    S4: Display,
{
    pub(crate) _locale: Locale,
    pub(crate) typst_docs: S1,
    pub(crate) inline_snippet: S2,
    pub(crate) author: S3,
    pub(crate) source_code: S4,
}

impl<S1, S2, S3, S4> FormatClarification<S1, S2, S3, S4>
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
                    self.typst_docs,
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
