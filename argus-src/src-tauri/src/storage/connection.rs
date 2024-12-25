use crate::structs::config::SYS_CONFIG;
use crate::utils::{db_init_util, file_util};
use diesel::connection::SimpleConnection;
use diesel::sqlite::SqliteConnection;
use diesel::{Connection, QueryResult, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::{env, fs};

/// 获取所有的数据库迁移
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    init_path().expect("数据库路径初始化失败!");
    let lazy = DATABASE_URL.as_str();
    let mut connection = SqliteConnection::establish(lazy)?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("TODO: panic message");
    Ok(())
}

/// 验证指定表是否存在
pub fn does_table_exist(
    conn: &mut SqliteConnection,
    table_name: &str,
) -> Result<bool, diesel::result::Error> {
    let query = format!(
        "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='{}')",
        table_name
    );
    let exists: QueryResult<bool> =
        diesel::dsl::sql::<diesel::sql_types::Bool>(&*query).get_result(conn);
    log::info!("query ans : {:?}", exists);

    Ok(exists?)
}

/// 全局惰性变量
pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    // 获取数据库保存路径
    let database_path = SYS_CONFIG.database_default_link.clone().unwrap();
    // 数据库路径
    let db_dir = SYS_CONFIG.database_path.clone().unwrap();
    let db_name = SYS_CONFIG.database_name.clone().unwrap();
    // 运行时根据环境变量计算
    let database_url = env::var(crate::constant::DATABASE_URL_KEY)
        .unwrap_or_else(|_| database_path.parse().unwrap());
    let cpath = file_util::get_root_folder().expect("根路径读取失败！ ");

    log::info!("Using database url: {:?},{}", cpath, database_url);
    let string = format!("{}/{}/{}", cpath.to_string_lossy(), db_dir, db_name);
    log::info!("Using database - url: {:?}", string);
    string
});

/// 初始化数据库路径
pub fn init_path() -> Result<(), rusqlite::Error> {
    let db_dir = SYS_CONFIG.database_path.clone().unwrap();
    // 使用 Tauri 提供的路径 API 确保数据库文件存放在用户目录下
    let cpath = file_util::get_root_folder().expect("TODO: panic message");
    let app_dir = format!("{}/{}", cpath.to_string_lossy(), db_dir);

    // 如果文件不存在，创建文件，反之无动作
    let exists = file_util::file_exists(&app_dir);
    if exists {
        log::info!("Found existing database url: {}", app_dir);
    } else {
        log::info!("Creating database directory: {}", app_dir);
        // 获取推荐的数据库路径
        fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    }

    log::info!("Initializing database connection");
    Ok(())
}

/// 创建数据库链接
pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&DATABASE_URL).unwrap_or_else(|err| {
        log::error!("Error connecting to {:?}: {:?}", *DATABASE_URL, err);
        panic!("Error connecting to {:?}: {:?}", *DATABASE_URL, err)
    });
    SqliteConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|err| panic!("Error connecting to {:?}: {:?}", *DATABASE_URL, err))
}

/// 删除指定表
pub fn drop_table(
    conn: &mut SqliteConnection,
    table_name: &str,
) -> Result<(), diesel::result::Error> {
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    diesel::sql_query(query).execute(conn)?;
    Ok(())
}

/// 初始化数据库【未使用】
fn init_databases() {
    let vec = db_init_util::get_init_sql_list();
    let mut connection = establish_connection();
    for x in vec {
        let is_exist = does_table_exist(&mut connection, &*x.name).unwrap();
        // 如果不存在则创建对应数据库
        if !is_exist {
            let result = diesel::sql_query(x.sql).execute(&mut connection);
            if !result.is_ok() {
                log::error!("Failed to create table: {:?}", x.name);
                panic!("Failed to create table: {:?}", x.name);
            }
        }
    }
    // 为数据库字段插入数据

    return;
    // todo: 2024/12/8 10:33 配置管理数据库升级
    SqliteConnection::batch_execute(
        &mut connection,
        "
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

    ",
    )
    .expect("TODO: panic message");
}
