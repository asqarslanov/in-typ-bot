mod root;

#[derive(Default, Clone, Copy)]
pub enum Locale {
    #[default]
    EnUs,
    RuRu,
}

pub fn locale(name: Locale) -> root::Translation {
    root::Translation {
        _locale: name,
        commands: root::s_commands::Section {
            _locale: name,
            help: root::s_commands::s_help::Section { _locale: name },
        },
    }
}
