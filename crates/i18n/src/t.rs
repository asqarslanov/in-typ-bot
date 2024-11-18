use crate::Locale;

pub mod commands;

pub struct Translation {
    pub(crate) __locale: Locale,
    /// - `.start()`
    /// - `.help()`
    pub commands: commands::Section,
}
