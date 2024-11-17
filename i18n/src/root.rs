use crate::Locale;

pub mod s_commands;

pub struct Translation {
    pub(crate) _locale: Locale,
    pub commands: s_commands::Section,
}
