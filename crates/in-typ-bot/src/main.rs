use crate::config::Config;

mod bot;
mod config;
mod logic;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    let config = Config::from_env()?;

    Box::pin(bot::run(config)).await;
    Ok(())
}
