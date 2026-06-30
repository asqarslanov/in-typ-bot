use std::env;

use eyre::Context as _;
use teloxide::types::ChatId;

pub struct Config {
    pub teloxide_token: String,
    pub cache_chat_id: ChatId,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        const TELOXIDE_TOKEN: &str = "TELOXIDE_TOKEN";
        const CACHE_CHAT_ID: &str = "CACHE_CHAT_ID";

        let teloxide_token =
            env::var(TELOXIDE_TOKEN).wrap_err_with(|| format!("{TELOXIDE_TOKEN} not found"))?;
        let cache_chat_id = ChatId(
            env::var(CACHE_CHAT_ID)
                .wrap_err_with(|| format!("{CACHE_CHAT_ID} not found"))?
                .parse::<i64>()
                .wrap_err_with(|| format!("{CACHE_CHAT_ID} should be an i64"))?,
        );
        Ok(Self {
            teloxide_token,
            cache_chat_id,
        })
    }
}
