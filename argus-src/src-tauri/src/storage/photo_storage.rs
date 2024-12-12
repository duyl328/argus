use crate::errors::SqlError;
use crate::models::photo_storage::PhotoStorage;
use crate::storage::schema::photo_storages::dsl::photo_storages;
use crate::storage::schema::posts::is_delete;
use crate::utils::time_util::TimeUtils;
use diesel::associations::HasTable;
use diesel::prelude::*;

/// 获取数据
pub fn get_all_photo_path(connection: &mut SqliteConnection) -> Result<Vec<PhotoStorage>, SqlError> {
    // 尝试加载所有数据
    let results = photo_storages
        .select(PhotoStorage::as_select()).filter(is_delete.eq(false))
        .load(connection)?;

    Ok(results)
}

/// 插入元素
pub fn insert_basic_setting(connection: &mut SqliteConnection, item: &mut PhotoStorage) -> Result<(), SqlError> {
    let result = get_all_photo_path(connection)?;
    let photo_path = item.img_paths.trim();
    for x in result {
        // 路径为空不能添加、路径重复不能添加
        if photo_path.is_empty() || x.img_paths.eq(&photo_path)  {
            return Err(SqlError::InsertError(String::from("Empty photo storage")));
        }
    }
    // 到这里数据合法，为必要数据重新赋值
    item.img_paths = photo_path.to_owned();
    let timestamp = TimeUtils::current_timestamp();
    item.create_time = timestamp;
    item.update_time = timestamp;
    item.is_enable = true;

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
