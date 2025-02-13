use crate::constant::IMAGE_COMPRESSION_STORAGE_FORMAT;
use crate::models::photo::{NewExifPhoto, NewPhoto, Photo};
use crate::storage::schema::photo_table::dsl::photo_table;
use crate::storage::schema::photo_table::{hash, is_delete};
use crate::utils::exif_utils::tag::{ExifInfo, ImgExif};
use crate::utils::img_util::ImageOperate;
use crate::utils::time_util::TimeUtils;
use anyhow::{anyhow, Result};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection, TextExpressionMethods};
use crate::storage::schema::photo_storages;
// 获取图片 hash、基础信息（长、宽、比例）、exif 信息

/// 把照片存储到数据库
pub fn insert_photo(connection: &mut SqliteConnection, img_info: ImageOperate) -> Result<()> {
    let photos = search_photo_by_hash(connection, img_info.hash.clone()).expect("查询出错");
    log::debug!("找到 {} 照片", photos.len());

    let op = if let Some(x) = img_info.format {
        x.to_mime_type()
    } else {
        ""
    };
    let timestamp = TimeUtils::current_timestamp();
    let np = NewPhoto {
        img_path: img_info.img_path,
        img_name: img_info.img_name,
        hash: img_info.hash,
        width: img_info.width,
        height: img_info.height,
        aspect_ratio: img_info.aspect_ratio,
        file_size: img_info.file_size,
        format: op.to_string(),
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
    };
}

/// 把照片存储及信息到数据库
/// - 数据库连结
/// - 图像信息
/// - exif 信息1
pub fn insert_photo_and_info(
    connection: &mut SqliteConnection,
    img_info: ImageOperate,
    img_exif: ImgExif,
) -> Result<()> {
    let photos = search_photo_by_hash(connection, img_info.hash.clone()).expect("查询出错");
    log::debug!("找到 {} 照片", photos.len());

    let op = if let Some(x) = img_info.format {
        x.to_mime_type()
    } else {
        ""
    };
    let timestamp = TimeUtils::current_timestamp();

    // 信息整理
    let gps_op: Option<String> = img_exif.gps_info.map(|info| info.to_string());
    let image_height_op = img_exif.image_height.map(|info| info as i32);
    let image_width_op = img_exif.image_width.map(|info| info as i32);
    let rating_op = img_exif.rating.map(|info| info as i32);
    let exposure_time_op = img_exif.exposure_time.map(|info| info as f32);
    let f_number_op = img_exif.f_number.map(|info| info as f32);
    let iso_op = img_exif.iso.map(|info| info as i32);
    let date_time_original_op = img_exif.date_time_original.map(|info| info.timestamp());
    let focal_length_op = img_exif.focal_length.map(|info| info as f32);
    let np = NewExifPhoto {
        img_path: img_info.img_path,
        img_name: img_info.img_name,
        hash: img_info.hash,
        width: img_info.width,
        height: img_info.height,
        aspect_ratio: img_info.aspect_ratio,
        file_size: img_info.file_size,
        format: op.to_string(),
        notes: None,
        is_algorithm: None,
        algorithm_score: None,
        create_time: timestamp,
        update_time: timestamp,

        offset_time: None,
        rating: rating_op,
        make: img_exif.make,
        model: img_exif.model,
        software: img_exif.software,
        exposure_time: exposure_time_op,
        flash: img_exif.flash,
        f_number: f_number_op,
        iso: iso_op,
        date_time_original: date_time_original_op,
        max_aperture_value: img_exif.max_aperture_value,
        focal_length: focal_length_op,
        image_width: image_width_op,
        image_height: image_height_op,
        gps_info: gps_op,
        exposure_program: img_exif.exposure_program,
        metering_mode: img_exif.metering_mode,
        artist: img_exif.artist,
        last_viewed_time: None,
        is_delete: false,
    };
    // 如果数据为空，添加该数据
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
        // 更新数据
        let res = diesel::update(photo_table.filter(hash.eq(np.hash.clone())))
            .set(np).execute(connection);
        if res.is_ok() {
            Ok(())
        } else {
            Err(anyhow!(res.unwrap_err()))
        }
    };
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
