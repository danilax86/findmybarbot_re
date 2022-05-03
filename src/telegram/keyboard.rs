use telexide::model::{KeyboardButton, ReplyMarkup};
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

pub fn build_reply_keyboard() -> ReplyMarkup {
    ReplyMarkup::ReplyKeyboardMarkup(ReplyKeyboardMarkup {
        keyboard: build_keyboard(),
        resize_keyboard: true,
        one_time_keyboard: false,
        selective: false,
    })
}