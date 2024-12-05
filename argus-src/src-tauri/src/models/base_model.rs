use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct BaseModel {
    pub id: i32,
    pub is_delete: bool,
    pub create_time: i64,
    pub update_time: i64,
}

/*

#[derive(Debug, Queryable)]
pub struct User {
    pub base: BaseEntity,
    pub name: String,
    pub email: String,
}

let user = User {
    base: BaseEntity {
        id: 1,
        is_delete: false,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    },
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};
println!("User ID: {}, Name: {}", user.base.id, user.name);

*/
