//! The best internationalization library ever.

mod shared;
mod t;

#[derive(Default, Clone, Copy)]
pub enum Locale {
    /// English (United States)
    #[default]
    EnUs,
    /// Russian (Russia)
    RuRu,
}

/// - `.commands`
/// - `.service`
#[must_use]
pub const fn t(locale: Locale) -> t::Translation {
    t::Translation {
        __locale: locale,
        commands: t::commands::Section { __locale: locale },
        service: t::service::Section { __locale: locale },
    }
}
