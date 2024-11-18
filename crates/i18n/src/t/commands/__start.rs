use std::fmt::Display;

use indoc::{formatdoc, indoc};

use crate::Locale;

pub struct FormatSnippet {
    pub(crate) __locale: Locale,
}

impl FormatSnippet {
    pub fn format_snippet<S: Display>(self, f: impl FnOnce(&str) -> S) -> String {
        match self.__locale {
            Locale::EnUs => formatdoc! {
                "
                    Hello! I am {}!
                    Send me some Typst code, and I will render it here.

                    For example:
                    {}

                    See {} for more details.\
                ",
                crate::shared::bot::USERNAME,
                f(indoc! {"
                    === Euler's identity:
                    #let exponent = $i pi$
                    $e^exponent + 1 = 0$\
                "}),
                crate::shared::commands::HELP
            },
            Locale::RuRu => formatdoc! {
                "
                    Привет! Я {}!
                    Отправьте мне код Typst, и я отрисую его здесь.

                    Например:
                    {}

                    Смотрите {} для подробностей.\
                ",
                crate::shared::bot::USERNAME,
                f(indoc! {"
                    === Тождество Эйлера:
                    #let exponent = $i pi$
                    $e^exponent + 1 = 0$\
                "}),
                crate::shared::commands::HELP
            },
        }
    }
}
