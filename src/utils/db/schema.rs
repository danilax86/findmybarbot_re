use diesel::table;
table! {
    places (id) {
        id -> Integer,
        name -> Varchar,
        lat -> Double,
        lng -> Double,
        description -> Varchar,
        address -> Varchar,
        img_url -> Varchar,
    }
}
