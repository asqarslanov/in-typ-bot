use teloxide::RequestError;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use tracing::info;

use self::handlers::commands::Command;
use crate::config::Config;

mod common;
mod handlers;

pub async fn run(config: Config) {
    let bot = Bot::new(config.teloxide_token);

    let mut dispatcher = Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![config.cache_chat_id])
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
