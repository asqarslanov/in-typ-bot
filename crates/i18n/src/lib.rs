mod t;

#[derive(Default, Clone, Copy)]
pub enum Locale {
    #[default]
    EnUs,
    RuRu,
}

pub fn locale(name: Locale) -> t::Translation {
    t::Translation {
        _locale: name,
        commands: t::commands::Section {
            _locale: name,
            help: t::commands::help::Section { _locale: name },
        },
    }
}
