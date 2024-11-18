use crate::Locale;

pub(crate) mod _help;

pub struct Section {
    pub(crate) _locale: Locale,
}

impl Section {
    pub const fn help(&self) -> _help::AddTypstDocs {
        _help::AddTypstDocs {
            _locale: self._locale,
        }
    }
}
