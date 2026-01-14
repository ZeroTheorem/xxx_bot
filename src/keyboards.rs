use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_main_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("👌👈 +1", "add")],
        vec![InlineKeyboardButton::callback(
            "🔞Итоги месяца!",
            "month_total",
        )],
        vec![InlineKeyboardButton::callback(
            "🔞Итоги года!",
            "year_total",
        )],
    ])
}

pub fn make_month_sub_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Итоги 🔞 за другой месяц!",
            "certain_month_total",
        )],
        vec![InlineKeyboardButton::callback("Назад ◀️", "back_to_main")],
    ])
}

pub fn make_year_sub_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Итоги 🔞 за другой год!",
            "certain_year_total",
        )],
        vec![InlineKeyboardButton::callback("Назад ◀️", "back_to_main")],
    ])
}
