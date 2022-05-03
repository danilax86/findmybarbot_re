use diesel::Queryable;

#[derive(Queryable)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub lat: f32,
    pub lng: f32,
    pub description: String,
    pub address: String,
    pub img_url: String,
}
