// @generated automatically by Diesel CLI.

diesel::table! {
    reviews (id) {
        id -> Integer,
        title -> Text,
        rating -> Float,
        time_date -> Date,
    }
}
