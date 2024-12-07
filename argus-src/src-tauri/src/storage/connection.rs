use diesel::{Connection, SqliteConnection};

use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::{env, fs};
use diesel::connection::SimpleConnection;
use crate::utils::file_util;

/// 全局惰性变量
pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok(); 
    // 运行时根据环境变量计算
    let database_url = env::var(crate::constant::DATABASE_URL_KEY)
        .unwrap_or_else(|_| crate::constant::DATABASE_DEFAULT_LINK.parse().unwrap());
    let cpath = env::current_dir().expect("TODO: panic message");

    log::info!("Using database url: {:?},{}", cpath,database_url);
    let string = format!("{}/{}/{}", cpath.to_string_lossy(), crate::constant::DATABASE_PATH, crate::constant::DATABASE_NAME);
    log::info!("Using database url: {:?}", string);
    string
    // init_database(&establish_connection());
    // database_url
});

pub fn init_database() -> Result<(), rusqlite::Error> {

    // 使用 Tauri 提供的路径 API 确保数据库文件存放在用户目录下
    let cpath = env::current_dir().expect("TODO: panic message");
    let app_dir = format!("{}/{}", cpath.to_string_lossy(), crate::constant::DATABASE_PATH);

    // 如果文件不存在，创建文件，反之无动作
    let exists = file_util::file_exists(&app_dir);
    if exists {
        log::info!("Found existing database url: {}", app_dir);
        return Ok(());
    }
    log::info!("Creating database directory: {}", app_dir);
    // 获取推荐的数据库路径
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    
    log::info!("Initializing database connection");
    init_databases();
    Ok(())
}

/// 创建数据库链接
pub fn establish_connection() -> SqliteConnection {
    log::info!("Establishing database connection");
    

    SqliteConnection::establish(&DATABASE_URL).unwrap_or_else(|err| {
        log::error!("Error connecting to {:?}: {:?}", *DATABASE_URL, err);
        panic!("Error connecting to {:?}: {:?}", *DATABASE_URL, err)
    });
    SqliteConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|err| panic!("Error connecting to {:?}: {:?}", *DATABASE_URL, err))
}

/// 初始化数据库
fn init_databases(){
    let mut connection = establish_connection();
    SqliteConnection::batch_execute(&mut connection, "
    create table __diesel_schema_migrations
(
    version VARCHAR(50)                         not null
        primary key,
    run_on  TIMESTAMP default CURRENT_TIMESTAMP not null
);

create table posts
(
    id          INTEGER               not null
        primary key autoincrement,
    title       VARCHAR               not null,
    body        TEXT                  not null,
    published   BOOLEAN default 0     not null,
    create_time BIGINT  default 0     not null,
    update_time BIGINT  default 0     not null,
    is_delete   BOOLEAN default FALSE not null
);

create table user
(
    id   INTEGER
        primary key autoincrement,
    name TEXT not null,
    age  INTEGER
);

    ").expect("TODO: panic message");
}