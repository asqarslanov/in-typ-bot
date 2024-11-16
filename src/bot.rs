use dotenvy_macro::dotenv;
use teloxide::dispatching::{
    dialogue::{self, InMemStorage},
    UpdateHandler,
};
use teloxide::prelude::*;

mod handlers;

pub async fn start() {
    let bot = Bot::from_env();

    let cache_chat = ChatId(
        dotenv!("CACHE_CHAT_ID")
            .to_owned()
            .parse::<i64>()
            .expect("chat id should be an i64"),
    );

    Box::pin(
        Dispatcher::builder(bot, {
            let _ = schema();
            Update::filter_inline_query().branch(dptree::endpoint(handlers::process_inline))
        })
        .dependencies(dptree::deps![cache_chat, InMemStorage::<()>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch(),
    )
    .await;
}

fn schema() -> UpdateHandler<()> {
    let inline_handler =
        Update::filter_inline_query().branch(dptree::endpoint(handlers::process_inline));

    let message_handler =
        Update::filter_message().branch(dptree::endpoint(handlers::process_message));

    dialogue::enter::<Update, InMemStorage<()>, _, _>()
        .branch(message_handler)
        .branch(inline_handler)
}
