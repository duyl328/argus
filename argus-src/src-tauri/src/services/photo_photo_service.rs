use std::collections::HashSet;
use crate::errors::SqlError;
use crate::models::photo_storage::PhotoStorage;
use crate::storage;
use crate::storage::connection::establish_connection;
use crate::utils::json_util::JsonUtil;
use serde_json::{self, Value, Error};

pub fn get_photo_storages() -> Result<Vec<PhotoStorage>, SqlError> {
    let mut conn = establish_connection();
    let result = storage::photo_storage::get_all_photo_path(&mut conn);
    if result.is_ok() {
        Ok(result?)
    } else {
        Err(result.unwrap_err())
    }
}

/// 更新图像地址【直接传输整个路径】
pub fn update_basic_setting_img_path(mut photo_path: PhotoStorage) -> Result<(), SqlError> {
    let mut conn = establish_connection();
    let result = storage::photo_storage::update_photo_storages(&mut conn, &mut photo_path)?;
    Ok(())
}

/// 添加一个图片路径
pub fn add_img_path(img_path: String, is_enable: bool) -> Result<(), SqlError> {
    let mut conn = establish_connection();
    storage::photo_storage::insert_img_path(&mut conn, img_path, is_enable)?;
    Ok(())
}
pub fn delete_img_path(id: i32) -> Result<(), SqlError> {
    let mut conn = establish_connection();
    storage::photo_storage::delete_img_path(&mut conn, id)?;
    Ok(())
}
