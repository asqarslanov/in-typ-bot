mod shared;
mod t;

#[derive(Default, Clone, Copy)]
pub enum Locale {
    #[default]
    EnUs,
    RuRu,
}

#[must_use]
pub const fn locale(locale: Locale) -> t::Translation {
    t::Translation {
        __locale: locale,
        commands: t::commands::Section { __locale: locale },
    }
}
