use diesel::prelude::*;
use tauri::http::Version;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::storage::schema::db_version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbVersion {
    pub id: i32,
    pub create_time: i64,
    pub update_time: i64,
    pub version: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::db_version)]
pub struct NewDbVersion<'a> {
    pub version: &'a i32,
}
