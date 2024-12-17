use crate::errors::SqlError;
use crate::models::photo_storage::{NewPhotoStorage, PhotoStorage};
use crate::storage::schema::photo_storages::dsl::photo_storages;
use crate::utils::time_util::TimeUtils;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::storage::schema::photo_storages::is_delete;

/// 获取数据
pub fn get_all_photo_path(connection: &mut SqliteConnection) -> Result<Vec<PhotoStorage>, SqlError> {
    // 尝试加载所有数据
    // let results = photo_storages::table
    //     .select(PhotoStorage::as_select()).filter(is_delete.eq(false))
    //     .load(connection)?;
    let results = photo_storages
        .filter(is_delete.eq(false)) // 过滤条件
        .load::<PhotoStorage>(connection)?;

    Ok(results)
}

/// 插入元素
pub fn insert_img_path(connection: &mut SqliteConnection, img_path: String, is_enable: bool) -> Result<(), SqlError> {
    let result = get_all_photo_path(connection)?;
    let photo_path = img_path.trim();
    for x in result {
        if photo_path.is_empty() {
            return Err(SqlError::InsertError(String::from("Empty photo storage!")));
        }
        if x.img_paths.eq(photo_path) {
            return Err(SqlError::InsertError(String::from("选择路径重复! ")));
        }
    }

    let timestamp = TimeUtils::current_timestamp();
    let item = NewPhotoStorage {
        img_paths: &photo_path.to_owned(),
        is_enable: &is_enable,
        is_delete: &false,
        create_time: &timestamp,
        update_time: &timestamp,
    };

    let result = diesel::insert_into(photo_storages::table())
        .values(item)
        .returning(PhotoStorage::as_returning())
        .get_result(connection);
    if result.is_ok() {
        Ok(())
    } else {
        Err(SqlError::Error(result.unwrap_err()))
    }
}

/// 更新图像存储路径
pub fn update_photo_storages(
    connection: &mut SqliteConnection,
    item: &mut PhotoStorage,
) -> Result<(), SqlError> {
    use crate::storage::schema::photo_storages;
    let result = diesel::update(photo_storages::table.filter(photo_storages::id.eq(item.id)))
        .set((
            photo_storages::img_paths.eq(&item.img_paths),
            photo_storages::update_time.eq(TimeUtils::current_timestamp()),
            photo_storages::is_enable.eq(item.is_enable),
        ))
        .execute(connection);

    match result {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                Ok(())
            } else {
                Err(SqlError::NotFound())
            }
        }
        Err(err) => Err(SqlError::Error(err)),
    }
}

/// 删除一个图像路径
pub fn delete_img_path(connection: &mut SqliteConnection,id:i32) -> Result<(), SqlError> {
    use crate::storage::schema::photo_storages;
    let result = diesel::update(photo_storages::table.filter(photo_storages::id.eq(id)))
        .set((
            photo_storages::update_time.eq(TimeUtils::current_timestamp()),
            photo_storages::is_delete.eq(true),
        ))
        .execute(connection);

    match result {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                Ok(())
            } else {
                Err(SqlError::NotFound())
            }
        }
        Err(err) => Err(SqlError::Error(err)),
    }
}


#[test]
/// 在内存中使用数据库测试【未测试】
fn test_with_memory_db() {
    use rusqlite::{Connection};
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
