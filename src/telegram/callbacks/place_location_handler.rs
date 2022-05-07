use telexide::api::types::SendLocation;
use telexide::client::Context;

pub async fn handle_place_location(data: &String, chat_id: i64, context: &Context) {
    if data.contains("location") {
        let mut iter = data.split_ascii_whitespace();
        iter.next();
        let lat = iter.next().unwrap().parse().unwrap();
        let lng = iter.next().unwrap().parse().unwrap();

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
}