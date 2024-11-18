use crate::Locale;

pub(crate) mod help;

pub struct Section {
    pub(crate) _locale: Locale,
    pub help: help::Section,
}
