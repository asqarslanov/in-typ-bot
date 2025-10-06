use dotenvy_macro::dotenv;
use teloxide::RequestError;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use tracing::info;

use self::handlers::commands::Command;

mod common;
mod handlers;

pub async fn run() {
    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let cache_chat = ChatId(
        dotenv!("CACHE_CHAT_ID")
            .to_owned()
            .parse::<i64>()
            .expect("chat id should be an i64"),
    );

    let mut dispatcher = Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![cache_chat])
        .enable_ctrlc_handler()
        .build();

    info!("Starting the bot.");
    Box::pin(dispatcher.dispatch()).await;
}

fn schema() -> UpdateHandler<RequestError> {
    let inline_handler = Update::filter_inline_query().branch(dptree::endpoint(
        handlers::typst_requests::inline_query::handle,
    ));

    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .branch(dptree::endpoint(handlers::commands::handle));

    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(handlers::typst_requests::message::handle));

    dptree::entry()
        .branch(inline_handler)
        .branch(command_handler)
        .branch(message_handler)
}
