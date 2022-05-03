#[macro_use]
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use crate::utils::db::models::*;
use crate::utils::db::schema::places::dsl::*;
use diesel::{insert_into, RunQueryDsl};
use diesel::QueryDsl;
use crate::utils::poi::Poi;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_places() {
    let connection = &mut establish_connection();

    let results = places
        .load::<Place>(connection)
        .expect("Error loading places");

    println!("Displaying {} places", results.len());
    for place in results {
        println!("{}", place.name);
        println!("----------\n");
        println!("{}", place.address);
    }
}

pub fn create_place(poi: &Poi) -> Place {
    let connection = &mut establish_connection();

    let new_place = NewPlace {
        name: poi.name.as_str(),
        lat: &poi.location.x(),
        lng: &poi.location.y(),
        description: poi.description.as_str(),
        address: poi.address.as_str(),
    };

    insert_into(places)
        .values(&new_place)
        .get_result(connection)
        .expect("Error saving new place")
}


