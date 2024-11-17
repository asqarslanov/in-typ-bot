use std::fmt::Display;

use crate::Locale;

pub(crate) mod m_message;

pub struct Section {
    pub(crate) _locale: Locale,
}

impl Section {
    pub fn message<S: Display>(&self, inline_snippet: S) -> m_message::Edit<S> {
        m_message::Edit {
            _locale: self._locale,
            inline_snippet,
        }
    }
}
