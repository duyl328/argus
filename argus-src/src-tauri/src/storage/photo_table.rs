use crate::models::photo::{NewPhoto, Photo};
use crate::storage::schema::photo_table::dsl::photo_table;
use crate::storage::schema::photo_table::{hash, is_delete};
use crate::utils::img_util::ImageOperate;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection, TextExpressionMethods};
use diesel::associations::HasTable;
use crate::constant::IMAGE_COMPRESSION_STORAGE_FORMAT;
use crate::utils::time_util::TimeUtils;
// 获取图片 hash、基础信息（长、宽、比例）、exif 信息

/// 把照片存储到数据库
pub fn insert_photo(connection: &mut SqliteConnection, img_info: ImageOperate)->Result<()> {
    let photos = search_photo_by_hash(connection, img_info.hash.clone()).expect("查询出错");
    log::debug!("找到 {} 照片", photos.len());

    let op = if let Some(x) = img_info.format {
        x.to_mime_type()
    }else{
        ""
    };
    let timestamp = TimeUtils::current_timestamp();
    let np = NewPhoto{
        img_path:img_info.img_path ,
        img_name:img_info.img_name ,
        hash:img_info.hash ,
        width:img_info.width,
        height:img_info.height,
        aspect_ratio:img_info.aspect_ratio,
        file_size:img_info.file_size,
        format:  op.to_string(),
        create_time: timestamp,
        update_time: timestamp,
    };
    return if photos.is_empty() {
        let res = diesel::insert_into(photo_table::table())
            .values(np)
            .returning(Photo::as_returning())
            .get_result(connection);
        if res.is_ok() {
            Ok(())
        } else {
            Err(anyhow!(res.unwrap_err()))
        }
    } else {
        Ok(())
    }
}

/// 查询照片是否存在
pub fn search_photo_by_hash(
    connection: &mut SqliteConnection,
    hash_str: String,
) -> Result<Vec<Photo>> {
    let results = photo_table
        .filter(is_delete.eq(false))
        .filter(hash.like(hash_str))
        .load::<Photo>(connection)?;
    return Ok(results);
}

pub fn search_photo_by_file_path(
    connection: &mut SqliteConnection,
    file_path: String,
) -> Vec<Photo> {
    return Vec::new();
}
pub fn search_photo_by_file_name(
    connection: &mut SqliteConnection,
    file_name: String,
) -> Vec<Photo> {
    return Vec::new();
}
