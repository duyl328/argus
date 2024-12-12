// @generated automatically by Diesel CLI.

diesel::table! {
    photo_storages (id) {
        id -> Integer,
        img_paths -> Text,
        is_enable -> Bool,
        is_delete -> Bool,
        create_time -> BigInt,
        update_time -> BigInt,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        is_delete -> Bool,
        create_time -> BigInt,
        update_time -> BigInt,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    photo_storages,
    posts,
);
