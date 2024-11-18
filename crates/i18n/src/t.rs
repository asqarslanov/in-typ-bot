use crate::Locale;

pub mod commands;

pub struct Translation {
    pub(crate) _locale: Locale,
    pub commands: commands::Section,
}
