use std::fmt::format;
use telexide::model::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyMarkup};
use telexide::model::ReplyKeyboardMarkup;
use crate::telegram;

fn build_keyboard() -> Vec<Vec<KeyboardButton>> {
    let send_loaction_btn = KeyboardButton {
        text: "📍 Отправить своё местоположение".parse().unwrap(),
        request_contact: false,
        request_location: true,
        request_poll: None,
    };

    vec![vec![send_loaction_btn]]
}

pub fn build_reply_keyboard_markup() -> ReplyMarkup {
    ReplyMarkup::ReplyKeyboardMarkup(ReplyKeyboardMarkup {
        keyboard: build_keyboard(),
        resize_keyboard: true,
        one_time_keyboard: false,
        selective: false,
    })
}

fn build_inline_keyboard(data: String) -> Vec<Vec<InlineKeyboardButton>> {
    let place_location_btn = InlineKeyboardButton {
        text: "Где это?".to_string(),
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

pub fn build_inline_keyboard_markup(data: String) -> ReplyMarkup {
    ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
        inline_keyboard: build_inline_keyboard(data)
    })
}