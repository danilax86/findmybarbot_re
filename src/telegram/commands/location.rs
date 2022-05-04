use geo::point;
use telexide::api::types::{SendLocation, SendMessage};
use telexide::model::{MessageContent, UpdateContent};
use telexide::prelude::{CommandResult, Context, Message, Update};
use telexide::prelude::prepare_listener;
use crate::telegram::keyboard::{build_inline_keyboard_markup, build_reply_keyboard_markup};
use telexide::prelude::command;
use crate::get_places_filtered_by_distance;
use math::round;
use telexide::model::ParseMode::Markdown;

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
            reply_markup: Some(build_reply_keyboard_markup()),
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

    let user_point = point!(x: location.latitude , y: location.longitude);

    let radius: f64 = 5.0;

    let places = get_places_filtered_by_distance(&user_point, radius);

    for place in places {
        let res = context
            .api
            .send_message(SendMessage {
                chat_id: message.chat.get_id(),
                text: (
                    format!("*{}*\n\n{}\n\n📍{}\n📏 в {} км от вас",
                            place.0.name, place.0.description, place.0.address,
                            round::ceil(place.1, 3))
                ),
                parse_mode: Option::from(Markdown),
                enitites: None,
                disable_web_page_preview: false,
                disable_notification: false,
                reply_to_message_id: None,
                allow_sending_without_reply: false,
                reply_markup: Some(build_inline_keyboard_markup(format!("location {} {}", place.0.lat, place.0.lng))),
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
}

#[prepare_listener]
pub async fn callback_handler(context: Context, update: Update) {
    let callback = match update.content {
        UpdateContent::CallbackQuery(ref q) => q,
        _ => return,
    };

    let data = callback.data.clone().unwrap_or_else(|| "".to_string());
    let chat_id = callback.message.clone().unwrap().chat.get_id();

    let mut lat: f64 = 0.0;
    let mut lng: f64 = 0.0;

    if data.contains("location") {
        let mut iter = data.split_ascii_whitespace();
        iter.next();
        lat = iter.next().unwrap().parse().unwrap();
        lng = iter.next().unwrap().parse().unwrap();
    }

    let res = context
        .api
        .send_location(SendLocation {
            chat_id,
            latitude: lat,
            longitude: lng,
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
