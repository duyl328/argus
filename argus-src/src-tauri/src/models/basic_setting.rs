use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::storage::schema::basic_setting)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BasicSetting {
    /// 图像路径存储
    pub img_paths: String,
    pub id: i32,
    pub create_time: i64,
    pub update_time: i64,
}


#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::basic_setting)]
pub struct NewBasicSetting<'a> {
    pub img_paths: &'a str,
}
