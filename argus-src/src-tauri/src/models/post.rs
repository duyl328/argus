use diesel::prelude::*;

// 将生成从 SQL 查询加载结构体所需的所有代码。
// 将生成代码，以基于通过定义的表，基于你的模型类型构造匹配的 select 子句
#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::storage::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    // 默认不推荐使用嵌套结构，暂无更好解决方案，使用展平结构 2024年12月6日 15点03分
    // #[diesel(embed)] // 嵌套结构需要使用 embed
    // pub base: BaseModel,
    pub id: i32,
    pub is_delete: bool,
    pub create_time: i64,
    pub update_time: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
// C:\Users\charlatans\AppData\Local\tauri\WixTools
