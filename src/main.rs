mod telegram;
mod utils;

use std::env;
use geo::{coord, point};
use telexide::{api::types::SendMessage, prelude::*};
use crate::location::hanlde_location;
use crate::ping_pong::*;
use crate::location::*;
use crate::telegram::commands::*;
use crate::utils::db::connect::{create_place, get_places};
use crate::utils::poi::Poi;


#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");



    ClientBuilder::new()
        .set_token(&token)
        .set_framework(create_framework!(&bot_name, ping, send_location))
        .add_handler_func(hanlde_location)
        .build()
        .start()
        .await
}