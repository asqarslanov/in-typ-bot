mod bot;
mod logic;

#[tokio::main]
async fn main() -> Result<(), dotenvy::Error> {
    dotenvy::dotenv()?;

    Box::pin(bot::start()).await;

    Ok(())
}
