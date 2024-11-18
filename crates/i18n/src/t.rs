use crate::Locale;

pub mod commands;
pub mod service;

pub struct Translation {
    pub(crate) __locale: Locale,
    /// - `.start()`
    /// - `.help()`
    pub commands: commands::Section,
    /// - `.wait()`
    pub service: service::Section,
}
