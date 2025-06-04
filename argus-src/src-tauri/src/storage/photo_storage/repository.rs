use crate::storage::schema::photo_storages_table::dsl::photo_storages_table;
use crate::storage::schema::photo_storages_table::is_delete;
use crate::utils::time_util::TimeUtils;
use anyhow::{anyhow, Result};
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::storage::photo_storage::model::{NewPhotoStorageEntity, PhotoStorageEntity};

/// 获取数据
pub fn get_all_photo_path(connection: &mut SqliteConnection) -> Result<Vec<PhotoStorageEntity>> {
    // 尝试加载所有数据
    // let results = photo_storages_table::table
    //     .select(PhotoStorageMod::as_select()).filter(is_delete.eq(false))
    //     .load(connection)?;
    let results = photo_storages_table
        .filter(is_delete.eq(false)) // 过滤条件
        .load::<PhotoStorageEntity>(connection)?;

    Ok(results)
}

/// 插入元素
pub fn insert_img_path(
    connection: &mut SqliteConnection,
    img_path: String,
    is_enable: bool,
) -> Result<()> {
    let result = get_all_photo_path(connection)?;
    let photo_path = img_path.trim();
    for x in result {
        if photo_path.is_empty() {
            return Err(anyhow!("Empty photo storage!"));
        }
        if x.img_paths.eq(photo_path) {
            return Err(anyhow!("选择路径重复!"));
        }
    }

    let timestamp = TimeUtils::current_timestamp();
    let item = NewPhotoStorageEntity {
        img_paths: &photo_path.to_owned(),
        is_enable: &is_enable,
        is_delete: &false,
        create_time: &timestamp,
        update_time: &timestamp,
    };

    let result = diesel::insert_into(photo_storages_table::table())
        .values(item)
        .returning(PhotoStorageEntity::as_returning())
        .get_result(connection);
    if result.is_ok() {
        Ok(())
    } else {
        Err(anyhow!(result.unwrap_err()))
    }
}

/// 更新图像存储路径
pub fn update_photo_storages(
    connection: &mut SqliteConnection,
    item: &mut PhotoStorageEntity,
) -> Result<()> {
    use crate::storage::schema::photo_storages_table;
    let result = diesel::update(photo_storages_table::table.filter(photo_storages_table::id.eq(item.id)))
        .set((
            photo_storages_table::img_paths.eq(&item.img_paths),
            photo_storages_table::update_time.eq(TimeUtils::current_timestamp()),
            photo_storages_table::is_enable.eq(item.is_enable),
        ))
        .execute(connection);

    match result {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                Ok(())
            } else {
                Err(anyhow!("未找到！"))
            }
        }
        Err(err) => Err(anyhow!("数据库错误")),
    }
}

/// 删除一个图像路径
pub fn delete_img_path(connection: &mut SqliteConnection, id: i32) -> Result<()> {
    use crate::storage::schema::photo_storages_table;
    let result = diesel::update(photo_storages_table::table.filter(photo_storages_table::id.eq(id)))
        .set((
            photo_storages_table::update_time.eq(TimeUtils::current_timestamp()),
            photo_storages_table::is_delete.eq(true),
        ))
        .execute(connection);

    match result {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                Ok(())
            } else {
                Err(anyhow!("未知数据库错误"))
            }
        }
        Err(err) => Err(anyhow!("位置数据库错误")),
    }
}

#[test]
/// 在内存中使用数据库测试【未测试】
fn test_with_memory_db() {
    use rusqlite::Connection;
    // let conn = Connection::open_in_memory().unwrap();
    // conn.execute(
    //     "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",None
    // ).unwrap();
    //
    // conn.execute(
    //     "INSERT INTO test (name) VALUES (?1)",
    //     &[&"test_name"],
    // ).unwrap();
    //
    // let mut stmt = conn.prepare("SELECT name FROM test WHERE id = 1").unwrap();
    // let name: String = stmt.query_row(None, |row| row.get(0)).unwrap();
    //
    // assert_eq!(name, "test_name");
}
