use std::collections::VecDeque;
use diesel::sql_types::Integer;
use geo::point;
use telexide::api::types::{SendLocation, SendMessage};
use telexide::model::{MessageContent, UpdateContent};
use telexide::prelude::{CommandResult, Context, Message, Update};
use telexide::prelude::prepare_listener;
use crate::telegram::keyboard::{build_inline_keyboard_markup, build_inline_keyboard_where_is_it, build_keyboard_send_location, build_reply_keyboard_markup};
use telexide::prelude::command;
use crate::get_places_filtered_by_distance;
use math::round;
use telexide::model::ParseMode::Markdown;
use crate::utils::db::models::Place;

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
            reply_markup: Some(build_reply_keyboard_markup(build_keyboard_send_location())),
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

    let mut places: Vec<(Place, f64)> = get_places_filtered_by_distance(&user_point, radius);

    places.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut places = VecDeque::from(places);


    send_provided_amount_of_items(&mut places, 4, message.chat.get_id(), &context).await;
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

async fn send_tuple(mut tuple: Option<&(Place, f64)>, chat_id: i64, context: &Context) {
    let mut tuple = tuple.unwrap();
    let res = context
        .api
        .send_message(SendMessage {
            chat_id,
            text: (
                format!("*{}*\n\n{}\n\n📍{}\n📏 в {} км от вас",
                        tuple.0.name, tuple.0.description, tuple.0.address,
                        round::ceil(tuple.1, 3))
            ),
            parse_mode: Option::from(Markdown),
            enitites: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: Some(build_inline_keyboard_markup(
                build_inline_keyboard_where_is_it(
                    format!("location {} {}", tuple.0.lat, tuple.0.lng)))),
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


async fn send_provided_amount_of_items(source: &mut VecDeque<(Place, f64)>, amount: i32, chat_id: i64, context: &Context) {
    for i in 0..amount {
        if i == 4 {
            // add inline button more
            break;
        }
        send_tuple(source.front(), chat_id, context).await;
        source.pop_front();
    };
}