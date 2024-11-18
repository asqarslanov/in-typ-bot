use crate::Locale;

pub struct Section {
    pub(crate) __locale: Locale,
}

impl Section {
    /// ```text
    /// Please, wait a second…
    /// ```
    pub const fn wait(&self) -> &'static str {
        match self.__locale {
            Locale::EnUs => "Please, wait a second…",
            Locale::RuRu => "Пожалуйста, подождите…",
        }
    }
}
