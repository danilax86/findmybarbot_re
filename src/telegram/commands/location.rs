use std::collections::VecDeque;
use geo::{point, Point};
use telexide::api::types::{SendMessage};
use telexide::model::{MessageContent, UpdateContent};
use telexide::prelude::{CommandResult, Context, Message, Update};
use telexide::prelude::prepare_listener;
use crate::telegram::keyboard::{build_inline_keyboard_markup, build_inline_keyboard_more, build_inline_keyboard_where_is_it, build_keyboard_send_location, build_reply_keyboard_markup};
use telexide::prelude::command;
use crate::get_places_filtered_by_distance;
use math::round;
use telexide::model::ParseMode::Markdown;
use crate::utils::db::models::Place;

pub static RADIUS: f64 = 5.0;
static AMOUNT: u32 = 4;

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
    let mut places: Vec<(Place, f64)> = get_places_filtered_by_distance(&user_point);
    places.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut places: VecDeque<(Place, f64)> = VecDeque::from(places);

    send_provided_amount_of_items(&mut places, message.chat.get_id(), user_point, &context).await;
}

async fn send_place_info(tuple: Option<&(Place, f64)>, chat_id: i64, context: &Context) {
    let tuple = tuple.unwrap();
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

pub async fn send_provided_amount_of_items(source: &mut VecDeque<(Place, f64)>, chat_id: i64, user_point: Point<f64>, context: &Context) {
    let source_amount_start = source.len();

    let mut counter: u32 = 0;
    while counter != AMOUNT {
        send_place_info(source.front(), chat_id, context).await;
        source.pop_front();

        // If there is no more places around
        if source.len() == 0 {
            return;
        }

        counter += 1;
    };

    let pop_places_amount = source_amount_start - source.len();


    send_more_message(&pop_places_amount, chat_id, user_point, context).await;
}

async fn send_more_message(pop_places_amount: &usize, chat_id: i64, user_point: Point<f64>, context: &Context) {
    let res = context
        .api
        .send_message(SendMessage {
            chat_id,
            text: "Не нашли подходящий бар?".parse().unwrap(),
            parse_mode: Option::from(Markdown),
            enitites: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: Some(build_inline_keyboard_markup(
                build_inline_keyboard_more(format!("more {} {} {}",
                                                   pop_places_amount, user_point.x(), user_point.y())))),
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