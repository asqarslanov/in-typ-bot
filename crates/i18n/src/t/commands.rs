use crate::Locale;

pub mod __help;
pub mod __start;

pub struct Section {
    pub(crate) __locale: Locale,
}

impl Section {
    /// ```text
    /// Hello! I am @InTypeBot!
    /// Send me some Typst code, and I will render it here.
    ///
    /// For example:
    /// '''typst
    /// === Euler's identity:
    /// #let exponent = $i pi$
    /// $e^exponent + 1 = 0$
    /// '''
    ///
    /// See /help for more details.
    /// ```
    pub const fn start(&self) -> __start::FormatSnippet {
        __start::FormatSnippet {
            __locale: self.__locale,
        }
    }

    /// ```text
    /// I’m a bot that can render Typst markup in Telegram chats.
    ///
    /// If you’re not familiar with Typst syntax, refer to their official documentation: https://typst.app/docs/.
    ///
    /// To use me in inline mode, type the following inside any chat:
    /// @InTypeBot $2 + 2 = 5$
    /// …of course, you can write any other Typst code.
    ///
    /// Or you can just use me in chat mode by sending me messages directly.
    ///
    /// Author: @AsqArslanov
    /// Source code: github.com/asqarslanov/in-typ-bot
    /// ```
    pub const fn help(&self) -> __help::FormatInlineSnippet {
        __help::FormatInlineSnippet {
            __locale: self.__locale,
        }
    }
}
