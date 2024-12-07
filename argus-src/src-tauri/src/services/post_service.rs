use crate::models::post::Post;
use crate::storage::connection::establish_connection;
use crate::storage::post;
use crate::storage::schema::posts::dsl::posts;
use diesel::sql_types::Integer;

/// 获取所有评论
pub fn get_all_post() {
    let conn = &mut establish_connection();

    let vec = post::get_all_post(conn);
    if vec.len() > 0 {
        println!("Displaying {} posts", vec.len());
        for post in vec {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
    } else {
        println!("No posts");
    }
}

/// 插入评论
pub fn insert_post() {
    let conn = &mut establish_connection();
    let post1 = post::insert_post(conn, "默认标题", "默认Body");
    println!("Displaying post success {}", post1.title);
}
