mod bot_commands;
mod database;
mod handlers;
mod keyboards;
use dotenv::dotenv;
mod messages;
mod states;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::types::ParseMode;
use teloxide::{filter_command, prelude::*};

use crate::bot_commands::Commands;
use crate::states::State;

#[tokio::main]
async fn main() {
    // load env variables
    dotenv().ok();

    // create Bot
    let bot = Bot::from_env().parse_mode(ParseMode::Html);

    // Create connections pool
    let pool = database::get_pool().await;

    // create messages provider
    let msg_provider = messages::Messages::new();

    // Create table
    database::create_table_if_not_exists(&pool).await;

    let command_handler = filter_command::<Commands, _>()
        .branch(dptree::case![Commands::Menu])
        .endpoint(handlers::menu_handler);

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::case![State::ReceiveMonth].endpoint(handlers::recive_month))
        .branch(dptree::case![State::ReciveYear].endpoint(handlers::recive_year));

    let callback_query_hanler =
        Update::filter_callback_query().endpoint(handlers::callback_query_handler);

    Dispatcher::builder(
        bot,
        dptree::entry()
            .enter_dialogue::<Update, InMemStorage<State>, State>()
            .branch(message_handler)
            .branch(callback_query_hanler),
    )
    .dependencies(dptree::deps![
        pool,
        msg_provider,
        InMemStorage::<State>::new()
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
