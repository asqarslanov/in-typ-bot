use std::fmt::Display;

use crate::Locale;

pub(crate) mod _message;

pub struct Section {
    pub(crate) _locale: Locale,
}

impl Section {
    pub fn message<S1, S2, S3, S4>(
        &self,
        typst_documentation: S1,
        inline_snippet: S2,
        author: S3,
        source_code: S4,
    ) -> _message::Format<S1, S2, S3, S4>
    where
        S1: Display,
        S2: Display,
        S3: Display,
        S4: Display,
    {
        _message::Format {
            _locale: self._locale,
            typst_documentation,
            inline_snippet,
            author,
            source_code,
        }
    }
}
