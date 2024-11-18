use strum::EnumString;
use teloxide::prelude::*;
use teloxide::types::User;

#[derive(EnumString, Default)]
#[strum(serialize_all = "lowercase")]
pub enum BotLocale {
    #[default]
    En,
    Ru,
}

impl From<BotLocale> for i18n::Locale {
    fn from(value: BotLocale) -> Self {
        match value {
            BotLocale::En => Self::EnUs,
            BotLocale::Ru => Self::RuRu,
        }
    }
}

impl From<&User> for BotLocale {
    fn from(value: &User) -> Self {
        value
            .language_code
            .as_ref()
            .and_then(|code| Some(code.parse().unwrap_or_default()))
            .unwrap_or_default()
    }
}

impl From<&Message> for BotLocale {
    fn from(value: &Message) -> Self {
        value
            .from
            .as_ref()
            .and_then(|user| Some(user.into()))
            .unwrap_or_default()
    }
}
