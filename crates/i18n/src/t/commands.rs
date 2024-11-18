use crate::Locale;

pub(crate) mod _help;
pub(crate) mod _start;

pub struct Section {
    pub(crate) _locale: Locale,
}

impl Section {
    pub const fn start(&self) -> _start::FormatSnippet {
        _start::FormatSnippet {
            _locale: self._locale,
        }
    }

    pub const fn help(&self) -> _help::FormatInlineSnippet {
        _help::FormatInlineSnippet {
            _locale: self._locale,
        }
    }
}
