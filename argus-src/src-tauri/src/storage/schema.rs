// @generated automatically by Diesel CLI.

diesel::table! {
    basic_setting (id) {
        id -> Integer,
        img_paths -> Text,
        create_time -> BigInt,
        update_time -> BigInt,
    }
}

diesel::table! {
    db_version (id) {
        id -> Integer,
        version -> Integer,
        create_time -> BigInt,
        update_time -> BigInt,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        create_time -> BigInt,
        update_time -> BigInt,
        is_delete -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    basic_setting,
    db_version,
    posts,
);
