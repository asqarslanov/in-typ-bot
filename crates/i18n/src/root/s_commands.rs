use crate::Locale;

pub(crate) mod s_help;

pub struct Section {
    pub(crate) _locale: Locale,
    pub help: s_help::Section,
}
