mod bot;
mod logic;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    Box::pin(bot::run()).await;
}
