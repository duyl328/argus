// @generated automatically by Diesel CLI.

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

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        name -> Text,
        age -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    user,
);
