use std::sync::Arc;

use crate::database::Database;
use crate::keyboards;
use crate::messages::Messages;
use crate::states::State;
use teloxide::adaptors::DefaultParseMode;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn menu_handler(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    db: Arc<Database>,
    msg_provider: Arc<Messages>,
) -> HandlerResult {
    let count = db.get_total().await;
    bot.send_message(msg.chat.id, msg_provider.all_message(count))
        .reply_markup(keyboards::make_main_menu())
        .await?;
    Ok(())
}

pub async fn callback_query_handler(
    bot: DefaultParseMode<Bot>,
    q: CallbackQuery,
    msg_provider: Arc<Messages>,
    db: Arc<Database>,
    dialogue: Dialogue<State, InMemStorage<State>>,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    if let Some(ref data) = q.data {
        match data.as_str() {
            "add" => {
                if let Some(message) = q.message {
                    if let Err(err) = db.add_row().await {
                        bot.edit_message_text(
                            message.chat.id,
                            message.id,
                            msg_provider.error_message(&err.to_string()),
                        )
                        .reply_markup(keyboards::make_main_menu())
                        .await?;
                        return Ok(());
                    }
                    let count = db.get_total().await;
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.all_message(count),
                    )
                    .reply_markup(keyboards::make_main_menu())
                    .await?;
                    return Ok(());
                }
            }

            "month_total" => {
                if let Some(message) = q.message {
                    let count = db.get_total_by_month().await;
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
                    dialogue.update(State::ReceiveYear).await.unwrap();
                }
            }
            "year_total" => {
                if let Some(message) = q.message {
                    let count = db.get_total_by_year().await;
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
                if let Some(message) = q.message {
                    let count = db.get_total().await;
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.all_message(count),
                    )
                    .reply_markup(keyboards::make_main_menu())
                    .await?;
                }
            }
            "last_rows" => {
                if let Some(message) = q.message {
                    let rows = db.get_last_five_rows().await;
                    bot.edit_message_text(
                        message.chat.id,
                        message.id,
                        msg_provider.last_rows_message(rows),
                    )
                    .reply_markup(keyboards::make_back_button_markup())
                    .await?;
                }
            }
            "delete_row" => {
                if let Some(message) = q.message {
                    if let Err(err) = db.delete_last_row().await {
                        bot.edit_message_text(
                            message.chat.id,
                            message.id,
                            msg_provider.error_message(&err.to_string()),
                        )
                        .reply_markup(keyboards::make_main_menu())
                        .await?;
                        return Ok(());
                    }
                    let count = db.get_total().await;
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
    bot: DefaultParseMode<Bot>,
    dialogue: Dialogue<State, InMemStorage<State>>,
    db: Arc<Database>,
    msg_provider: Arc<Messages>,
    msg: Message,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        let month: i32 = match text.parse::<i32>() {
            Ok(i) => i,
            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    msg_provider.error_message("Пожалуйста введите число"),
                )
                .await?;
                return Ok(());
            }
        };
        if month > 12 || month <= 0 {
            bot.send_message(
                msg.chat.id,
                msg_provider.error_message("Такого месяца не существует, попробуйте снова"),
            )
            .await?;
            return Ok(());
        }
        let count = db.get_total_by_certain_month(month).await;
        bot.send_message(msg.chat.id, msg_provider.certain_month_message(count))
            .reply_markup(keyboards::make_main_menu())
            .await?;

        dialogue.exit().await?;
    }
    Ok(())
}

pub async fn recive_year(
    bot: DefaultParseMode<Bot>,
    dialogue: Dialogue<State, InMemStorage<State>>,
    db: Arc<Database>,
    msg: Message,
    msg_provider: Arc<Messages>,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        let year: i32 = match text.parse::<i32>() {
            Ok(i) => i,
            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    msg_provider.error_message("Пожалуйста введите число"),
                )
                .await?;
                return Ok(());
            }
        };
        let count = db.get_total_by_certain_year(year).await;
        bot.send_message(msg.chat.id, msg_provider.certain_year_message(year, count))
            .reply_markup(keyboards::make_main_menu())
            .await?;

        dialogue.exit().await?;
    }
    Ok(())
}
