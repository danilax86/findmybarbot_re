use telexide::prelude::prepare_listener;
use telexide::client::Context;
use telexide::model::{Update, UpdateContent};
use crate::telegram::callbacks::more_btn_handler::handle_more_place_btn;
use crate::telegram::callbacks::place_location_handler::handle_place_location;

#[prepare_listener]
pub async fn callback_handler(context: Context, update: Update) {
    let callback = match update.content {
        UpdateContent::CallbackQuery(ref q) => q,
        _ => return,
    };

    let data: String = callback.data.clone().unwrap_or_else(|| "".to_string());
    let chat_id: i64 = callback.message.clone().unwrap().chat.get_id();

    // Add callback handlers according to their data below this comment
    handle_place_location(&data, chat_id, &context).await;
    handle_more_place_btn(&data, chat_id, &context).await;
}

