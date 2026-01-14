use std::sync::Arc;

use crate::database;
use crate::keyboards;
use crate::messages::Messages;
use crate::states::State;
use sqlx::SqlitePool;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn menu_handler(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
    msg_provider: Arc<Messages>,
) -> HandlerResult {
    let count = database::get_all(&pool).await;
    bot.send_message(msg.chat.id, msg_provider.all_message(count))
        .reply_markup(keyboards::make_main_menu())
        .await?;
    Ok(())
}

pub async fn callback_query_handler(
    bot: Bot,
    q: CallbackQuery,
    pool: SqlitePool,
    msg_provider: Arc<Messages>,
    dialogue: Dialogue<State, InMemStorage<State>>,
) -> HandlerResult {
    if let Some(ref data) = q.data {
        match data.as_str() {
            "add" => {
                database::add_row(&pool).await;
                let count = database::get_all(&pool).await;
                if let Some(message) = q.message {
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.all_message(count),
                    )
                    .reply_markup(keyboards::make_main_menu())
                    .await?;
                }
            }
            "month_total" => {
                let count = database::get_all_by_month(&pool).await;
                if let Some(message) = q.message {
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.month_message(count),
                    )
                    .reply_markup(keyboards::make_month_sub_menu())
                    .await?;
                }
            }
            "certain_month_total" => {
                if let Some(message) = q.message {
                    bot.send_message(message.chat.id, "Введи месяц").await?;
                    dialogue.update(State::ReceiveMonth).await.unwrap();
                }
            }
            "certain_year_total" => {
                if let Some(message) = q.message {
                    bot.send_message(message.chat.id, "Введи год").await?;
                    dialogue.update(State::ReciveYear).await.unwrap();
                }
            }
            "year_total" => {
                let count = database::get_all_by_year(&pool).await;
                if let Some(message) = q.message {
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.year_message(count),
                    )
                    .reply_markup(keyboards::make_year_sub_menu())
                    .await?;
                }
            }
            "back_to_main" => {
                let count = database::get_all(&pool).await;
                if let Some(message) = q.message {
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.all_message(count),
                    )
                    .reply_markup(keyboards::make_main_menu())
                    .await?;
                }
            }
            _ => (),
        }
    }
    Ok(())
}
pub async fn recive_month(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        let month: i64 = match text.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                bot.send_message(msg.chat.id, "this is not integer").await?;
                return Ok(());
            }
        };
        if month > 12 || month < 0 {
            bot.send_message(msg.chat.id, "please type valid month")
                .await?;
            return Ok(());
        }
        dialogue.exit().await?;
    }
    Ok(())
}

pub async fn recive_year(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
    pool: SqlitePool,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        let year: i64 = match text.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                bot.send_message(msg.chat.id, "this is not integer").await?;
                return Ok(());
            }
        };
        let count = database::get_all_by_certain_year(&pool, year).await;
        bot.send_message(msg.chat.id, format!("this is not integer {}", count))
            .reply_markup(keyboards::make_main_menu())
            .await?;
        dialogue.exit().await?;
    }
    Ok(())
}
