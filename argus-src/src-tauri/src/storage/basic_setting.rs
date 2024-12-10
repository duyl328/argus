use crate::errors::SqlError;
use crate::models::basic_setting::BasicSetting;
use crate::storage::schema::basic_setting::dsl::basic_setting;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::constant::BASIC_SETTING_ID;
use crate::models;

/// 获取数据
pub fn get_basic_setting(connection: &mut SqliteConnection) -> Result<BasicSetting, SqlError> {
    // 尝试加载所有数据
    // let results: Vec<BasicSetting> = basic_setting
    //     .select(BasicSetting::as_select())
    //     .load(connection)?;
    let results = basic_setting
        .select(BasicSetting::as_select())
        .load(connection)?;

    match results.len() {
        0 => {
            // 没有数据，调用插入函数创建默认数据
            let default_setting = BasicSetting::default();
            insert_basic_setting(connection, &default_setting)?;
            Ok(default_setting)
        }
        1 => {
            // 只有一个数据，直接返回
            Ok(results.into_iter().next().unwrap())
        }
        _ => {
            // 超过一个数据，返回错误
            Err(SqlError::Error(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("Too many records found in basic_setting".to_string()),
            )))
        }
    }
}


/// 插入元素
pub fn insert_basic_setting(connection: &mut SqliteConnection, item: &BasicSetting) -> Result<(), SqlError> {
    let result = diesel::insert_into(basic_setting::table())
        .values(item)
        .returning(BasicSetting::as_returning())
        .get_result(connection);
    if result.is_ok() {
        Ok(())
    } else {
        Err(SqlError::Error(result.unwrap_err()))
    }
}
pub fn update_basic_setting(
    connection: &mut SqliteConnection,
    new_item: &mut BasicSetting,
) -> Result<(), SqlError> {
    use crate::storage::schema::basic_setting;
    new_item.id = BASIC_SETTING_ID;
    let result = diesel::update(basic_setting::table.filter(basic_setting::id.eq(BASIC_SETTING_ID)))
        .set((
            basic_setting::img_paths.eq(&new_item.img_paths),
            basic_setting::update_time.eq(&new_item.update_time),
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