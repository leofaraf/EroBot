// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Integer,
        title -> Text,
        photo_path -> Text,
    }
}

diesel::table! {
    post (id) {
        id -> Integer,
        person_id -> Integer,
        photo_path -> Text,
        is_premium -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    person,
    post,
);
