mod bot;
mod logic;

#[tokio::main]
async fn main() -> Result<(), dotenvy::Error> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;

    Box::pin(bot::start()).await;

    Ok(())
}
