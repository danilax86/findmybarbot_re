use telexide::model::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyMarkup};
use telexide::model::ReplyKeyboardMarkup;

pub fn build_keyboard_send_location() -> Vec<Vec<KeyboardButton>> {
    let send_loaction_btn = KeyboardButton {
        text: "📍 Отправить своё местоположение".parse().unwrap(),
        request_contact: false,
        request_location: true,
        request_poll: None,
    };

    vec![vec![send_loaction_btn]]
}

pub fn build_inline_keyboard_where_is_it(data: String) -> Vec<Vec<InlineKeyboardButton>> {
    let place_location_btn = InlineKeyboardButton {
        text: "🗺 Где это?".to_string(),
        url: Option::from("".to_string()),
        login_url: None,
        callback_data: Some(data),
        switch_inline_query: None,
        switch_inline_query_current_chat: None,
        callback_game: None,
        pay: false,
    };

    vec![vec![place_location_btn]]
}

pub fn build_inline_keyboard_more(data: String) -> Vec<Vec<InlineKeyboardButton>> {
    let place_location_btn = InlineKeyboardButton {
        text: "✨ Показать ещё".to_string(),
        url: Option::from("".to_string()),
        login_url: None,
        callback_data: Some(data),
        switch_inline_query: None,
        switch_inline_query_current_chat: None,
        callback_game: None,
        pay: false,
    };

    vec![vec![place_location_btn]]
}

pub fn build_reply_keyboard_markup(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyMarkup {
    ReplyMarkup::ReplyKeyboardMarkup(ReplyKeyboardMarkup {
        keyboard,
        resize_keyboard: true,
        one_time_keyboard: false,
        selective: false,
    })
}

pub fn build_inline_keyboard_markup(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
    ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
        inline_keyboard,
    })
}