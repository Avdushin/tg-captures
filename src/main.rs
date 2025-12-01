mod handlers;
mod types;
mod ui;

use dotenvy::dotenv;
use env_logger;
use log::info;
use teloxide::{dptree, prelude::*};

use crate::handlers::{handle_callback_query, handle_message};

#[tokio::main]
async fn main() {
    // Подхватываем .env
    dotenv().ok();
    env_logger::init();
    info!("Starting bot...");

    // Токен берётся из TELOXIDE_TOKEN
    let bot = Bot::from_env();

    // dptree-handler: обрабатываем и сообщения, и callback-запросы
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback_query));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}
