use std::collections::VecDeque;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use crate::utils::db::models::*;
use crate::utils::db::schema::places::dsl::*;
use diesel::{insert_into, RunQueryDsl};
use geo::Point;
use crate::utils::calculate::calculate_distance;


pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_places() -> Vec<Place> {
    let connection = &mut establish_connection();

    let results = places
        .load::<Place>(connection)
        .expect("Error loading places");

    results
}

pub fn get_places_filtered_by_distance(user_point: &Point<f64>, radius: f64) -> Vec<(Place, f64)> {
    let connection = &mut establish_connection();

    let results = places
        .load::<Place>(connection)
        .expect("Error loading places");

    let mut result: Vec<(Place, f64)> = vec![];


    for place in results {
        let distance = calculate_distance(user_point, &Point::new(place.lat, place.lng));
        if distance < radius {
            let _ = &result.push((place, distance));
        }
    }

    result
}

// add img_url
pub fn create_place(place: &Place) -> Place {
    let connection = &mut establish_connection();

    let new_place = NewPlace {
        name: place.name.as_str(),
        lat: &place.lat,
        lng: &place.lng,
        description: place.description.as_str(),
        address: place.address.as_str(),
    };

    insert_into(places)
        .values(&new_place)
        .get_result(connection)
        .expect("Error saving new place")
}


