use crate::Locale;

pub mod commands;

pub struct Translation {
    pub(crate) __locale: Locale,
    pub commands: commands::Section,
}
