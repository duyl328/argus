use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::utils::time_util::TimeUtils;

#[derive(Queryable, Selectable, Insertable, Debug,Serialize)]
#[diesel(table_name = crate::storage::schema::basic_setting)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BasicSetting {
    /// 图像路径存储
    pub id: i32,
    pub img_paths: String,
    pub create_time: i64,
    pub update_time: i64,
}

impl BasicSetting {
    /// 默认设置，提供默认值
    pub fn default() -> Self {
        Self {
            img_paths: String::from(""),
            id: crate::constant::BASIC_SETTING_ID, // 假设默认 ID 是 1，你可能需要确保唯一性
            create_time: TimeUtils::current_timestamp(),
            update_time: TimeUtils::current_timestamp(),
        }
    }
}

// #[derive(Insertable)]
// #[diesel(table_name = crate::storage::schema::basic_setting)]
// pub struct NewBasicSetting<'a> {
//     pub img_paths: &'a str,
//     pub create_time: i64,
//     pub update_time: i64,
// }
