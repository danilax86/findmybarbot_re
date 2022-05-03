use telexide::api::types::{CopyMessage, EditMessageReplyMarkup, SendLocation, SendMessage};
use telexide::model::{MessageContent, UpdateContent};
use telexide::prelude::{CommandResult, Context, Message, Update};
use telexide::prelude::prepare_listener;
use crate::telegram::keyboard::build_reply_keyboard;
use telexide::prelude::command;


#[command(description = "Отправить своё местоположение")]
async fn send_location(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage {
            chat_id: message.chat.get_id(),
            text: "Отправь своё местоположение".to_string(),
            parse_mode: None,
            enitites: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: Some(build_reply_keyboard()),
        })
        .await?;
    Ok(())
}


#[prepare_listener]
pub async fn hanlde_location(context: Context, update: Update) {
    let message = match update.content {
        UpdateContent::Message(ref m) => m,
        _ => return,
    };

    if message.from.is_none() {
        return;
    }

    let location = match message.content {
        MessageContent::Location {
            ref content, ..
        } => content,
        _ => return,
    };


    //todo: Отправить несколько сообщений с inline keyboard


    let res = context
        .api
        .send_location(SendLocation {
            chat_id: message.chat.get_id(),
            latitude: location.latitude,
            longitude: location.longitude,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
        .await;
    if res.is_err() {
        println!(
            "got an error when sending the asking message: {}",
            res.err().unwrap()
        );
        return;
    }
}