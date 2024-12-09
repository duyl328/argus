use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub struct DbMigration {
    pub name: String,
    pub sql: String,
    pub version: u32,
}


pub fn get_migrations() -> Vec<DbMigration> {
    let  migration_list: Vec<DbMigration> = vec![
        DbMigration{
            name: "".to_string(),
            sql: "".to_string(),
            version: 0,
        }
    ];
    migration_list
}