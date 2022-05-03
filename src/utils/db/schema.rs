diesel::table! {
    places (id) {
        id -> Int4,
        name -> Text,
        lat -> Float8,
        lng -> Float8,
        description -> Text,
        address -> Text,
        img_url -> Text,
    }
}
