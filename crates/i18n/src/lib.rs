mod shared;
mod t;

#[derive(Default, Clone, Copy)]
pub enum Locale {
    #[default]
    EnUs,
    RuRu,
}

#[must_use]
pub const fn locale(name: Locale) -> t::Translation {
    t::Translation {
        _locale: name,
        commands: t::commands::Section { _locale: name },
    }
}
