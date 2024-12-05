use std::sync::Mutex;
use once_cell::sync::Lazy;
use dotenvy::dotenv;
use std::env;
use diesel::{Connection, SqliteConnection};

/// 全局惰性变量
pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    // 运行时根据环境变量计算
    env::var(crate::constant::DATABASE_URL_KEY)
        .unwrap_or_else(|_| crate::constant::DATABASE_DEFAULT_LINK.parse().unwrap())
});

/// 创建数据库链接
pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|err| panic!("Error connecting to {:?}: {:?}", *DATABASE_URL, err))
}
