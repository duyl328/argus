use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::utils::time_util::TimeUtils;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::storage::schema::photo_storages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PhotoStorage {
    /// 图像路径存储
    pub id: i32,
    pub img_paths: String,
    pub is_enable: bool,
    pub is_delete: bool,
    pub create_time: i64,
    pub update_time: i64,
}

impl PhotoStorage {
    /// 默认设置，提供默认值
    pub fn default() -> Self {
        Self {
            img_paths: String::from(""),
            id: crate::constant::BASIC_SETTING_ID, // 假设默认 ID 是 1，你可能需要确保唯一性
            is_enable: false,
            create_time: TimeUtils::current_timestamp(),
            update_time: TimeUtils::current_timestamp(),
            is_delete: false,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::photo_storages)]
pub struct NewPhotoStorage<'a> {
    /// 图像路径存储
    pub img_paths: &'a String,
    pub is_enable: &'a bool,
    pub is_delete: &'a bool,
    pub create_time: &'a i64,
    pub update_time: &'a i64,
}

// #[derive(Insertable)]
// #[diesel(table_name = crate::storage::schema::photo_storages)]
// pub struct NewBasicSetting<'a> {
//     pub img_paths: &'a str,
//     pub create_time: i64,
//     pub update_time: i64,
// }
