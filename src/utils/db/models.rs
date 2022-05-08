use diesel::Queryable;
use diesel::Insertable;

use super::schema::*;

#[derive(Queryable)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lng: f64,
    pub description: String,
    pub address: String,
    pub img_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = places)]
pub struct NewPlace<'a> {
    pub name: &'a str,
    pub lat: &'a f64,
    pub lng: &'a f64,
    pub description: &'a str,
    pub address: &'a str,
}
