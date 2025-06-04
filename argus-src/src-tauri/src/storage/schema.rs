// @generated automatically by Diesel CLI.

diesel::table! {
    photo_storages_table (id) {
        id -> Integer,
        img_paths -> Text,
        is_enable -> Bool,
        is_delete -> Bool,
        create_time -> BigInt,
        update_time -> BigInt,
    }
}

diesel::table! {
    photo_table (id) {
        id -> Integer,
        img_path -> Text,
        img_name -> Text,
        hash -> Text,
        width -> Integer,
        height -> Integer,
        aspect_ratio -> Float,
        file_size -> BigInt,
        format -> Text,
        notes -> Nullable<Text>,
        make -> Nullable<Text>,
        model -> Nullable<Text>,
        software -> Nullable<Text>,
        exposure_time -> Nullable<Float>,
        flash -> Nullable<Text>,
        f_number -> Nullable<Float>,
        iso -> Nullable<Integer>,
        date_time_original -> Nullable<BigInt>,
        max_aperture_value -> Nullable<Text>,
        focal_length -> Nullable<Float>,
        image_width -> Nullable<Integer>,
        image_height -> Nullable<Integer>,
        gps_info -> Nullable<Text>,
        exposure_program -> Nullable<Text>,
        metering_mode -> Nullable<Text>,
        artist -> Nullable<Text>,
        is_delete -> Bool,
        create_time -> BigInt,
        update_time -> BigInt,
        is_algorithm -> Bool,
        algorithm_score -> Nullable<Integer>,
        last_viewed_time -> Nullable<BigInt>,
        offset_time -> Nullable<Text>,
        rating -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    photo_storages_table,
    photo_table,
);
