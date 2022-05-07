mod telegram;
mod utils;

use std::env;
use telexide::{prelude::*};
use telexide::api::types::UpdateType;
use crate::location::hanlde_location;
use crate::ping_pong::*;
use crate::location::*;
use crate::telegram::callbacks::callback_handler::callback_handler;
use crate::telegram::commands::*;
use crate::utils::db::methods::{create_place, get_places, get_places_filtered_by_distance};


#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    ClientBuilder::new()
        .set_token(&token)
        .set_framework(create_framework!(&bot_name, ping, send_location))
        .set_allowed_updates(vec![UpdateType::CallbackQuery, UpdateType::Message])
        .add_handler_func(hanlde_location)
        .add_handler_func(callback_handler)
        .build()
        .start()
        .await
}