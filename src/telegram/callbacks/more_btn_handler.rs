use std::collections::VecDeque;
use geo::point;
use telexide::client::Context;
use crate::{get_places_filtered_by_distance, send_provided_amount_of_items};
use crate::utils::db::models::Place;

pub async fn handle_more_place_btn(data: &String, chat_id: i64, context: &Context) {
    if data.contains("more") {
        let mut iter = data.split_ascii_whitespace();
        iter.next();

        let mut pop_places_amount: u32 = iter.next().unwrap().parse().unwrap();
        let user_x: f64 = iter.next().unwrap().parse().unwrap();
        let user_y: f64 = iter.next().unwrap().parse().unwrap();

        let user_point = point!(x: user_x, y: user_y);

        let mut places: Vec<(Place, f64)> = get_places_filtered_by_distance(&user_point);
        places.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut places: VecDeque<(Place, f64)> = VecDeque::from(places);

        while pop_places_amount != 0 {
            places.pop_front();
            pop_places_amount -= 1;
        }

        send_provided_amount_of_items(&mut places, chat_id, user_point, &context).await;
    }
}