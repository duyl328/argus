use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

#[derive(Debug,Clone)]
pub struct Conf {
    /// 数据库默认连接
    pub database_default_link: String,
    /// 数据库名称
    pub database_name: String,
    /// 数据库路径
    pub database_path: String,
    /// 缓存路径
    pub cache_path: String,
    /// 缩略图文件夹名称
    pub image_cache_path: String,
    /// 缩略图缓存真实路径
    pub thumbnail_storage_path: String,
    /// 时间默认格式
    pub time_basic_fmt: String,
    /// 目录界别
    pub directory_level: u32,
    /// Python 服务地址
    pub python_service_path: String,
}

pub(crate) static CONF: Lazy<Arc<RwLock<Conf>>> = Lazy::new(|| Arc::new(RwLock::new(Conf::default())));
pub(crate) static CONF_DEFAULT: Lazy<Conf> = Lazy::new(|| Conf::default());

impl Conf {
    pub fn default() -> Conf {
        Conf {
            database_default_link: "db/sqlite.db".to_string(),
            database_name: "sqlite.db".to_string(),
            database_path: "db".to_string(),
            image_cache_path: "compress".to_string(),
            cache_path: "cache".to_string(),
            thumbnail_storage_path: String::from(""),
            time_basic_fmt: "%Y-%m-%d %H:%M:%S".to_string(),
            directory_level: 3,
            python_service_path: String::from("http://127.0.0.1:5000/"),
        }
    }
}
