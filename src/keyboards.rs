use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

fn make_back_button() -> Vec<InlineKeyboardButton> {
    vec![InlineKeyboardButton::callback("Назад ◀️", "back_to_main")]
}

pub fn make_main_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("❤️", "add")],
        vec![InlineKeyboardButton::callback(
            "🔞 Итоги месяца!",
            "month_total",
        )],
        vec![InlineKeyboardButton::callback(
            "🔞 Итоги года!",
            "year_total",
        )],
        vec![InlineKeyboardButton::callback(
            "✍️ Последние записи",
            "last_rows",
        )],
        vec![InlineKeyboardButton::callback(
            "🗑 Удалить значение",
            "delete_row",
        )],
    ])
}

pub fn make_month_sub_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Итоги 🔞 за другой месяц!",
            "certain_month_total",
        )],
        make_back_button(),
    ])
}

pub fn make_year_sub_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Итоги 🔞 за другой год!",
            "certain_year_total",
        )],
        make_back_button(),
    ])
}

pub fn make_back_button_markup() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![make_back_button()])
}
