use crate::Locale;

pub mod __help;
pub mod __start;

pub struct Section {
    pub(crate) __locale: Locale,
}

impl Section {
    pub const fn start(&self) -> __start::FormatSnippet {
        __start::FormatSnippet {
            __locale: self.__locale,
        }
    }

    pub const fn help(&self) -> __help::FormatInlineSnippet {
        __help::FormatInlineSnippet {
            __locale: self.__locale,
        }
    }
}
