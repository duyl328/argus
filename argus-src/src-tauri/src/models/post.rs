use diesel::prelude::*;

// 将生成从 SQL 查询加载结构体所需的所有代码。
#[derive(Queryable, Selectable)]
// 将生成代码，以基于通过定义的表，基于你的模型类型构造匹配的 select 子句。
#[diesel(table_name = crate::storage::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
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
