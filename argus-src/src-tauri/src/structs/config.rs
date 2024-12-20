use crate::constant::{DEFAULT_PROFILE_NAME};
use crate::utils::file_util;
use crate::utils::file_util::{create_folder, get_root_folder};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// 数据库默认连接
    pub database_default_link: Option<String>,
    /// 数据库名称
    pub database_name: Option<String>,
    /// 数据库路径
    pub database_path: Option<String>,

    /// 缓存路径
    pub cache_path: Option<String>,
    /// 缩略图文件夹名称
    pub image_cache_path: Option<String>,
    /// 缩略图缓存真实路径
    pub thumbnail_storage_path: Option<String>,

    /// 时间默认格式
    pub time_basic_fmt: Option<String>,

    #[serde(flatten)] // 收集多余的字段
    extra: HashMap<String, String>,
}

impl Config {
    pub fn add_extra_field(&mut self, key: String, value: String) {
        self.extra.insert(key, value);
    }

    pub fn default() -> Config {
        Config {
            database_default_link: Some("db/sqlite.db".to_string()),
            database_name: Some("sqlite.db".to_string()),
            database_path: Some("db".to_string()),
            image_cache_path: Some("compress".to_string()),
            cache_path: Some("cache".to_string()),
            thumbnail_storage_path: None,
            time_basic_fmt: Some("%Y-%m-%d %H:%M:%S".to_string()),
            extra: HashMap::new(),
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.database_default_link == other.database_default_link
            && self.database_name == other.database_name
            && self.database_path == other.database_path
            && self.image_cache_path == other.image_cache_path
            && self.cache_path == other.cache_path
            && self.thumbnail_storage_path == other.thumbnail_storage_path
            && self.time_basic_fmt == other.time_basic_fmt
            && self.extra == other.extra
    }
}

pub static SYS_CONFIG: Lazy<Config> = Lazy::new(|| load_config().expect("系统配置文件加载失败! "));

/// 获取配置文件存放路径
fn get_config_dir() -> String {
    let root_dir = get_root_folder().expect("根路径获取失败! ");
    root_dir.join(DEFAULT_PROFILE_NAME).display().to_string()
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_dir();
    let toml_string = to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_dir();

    let mut config;

    // 检查配置文件是否存在
    if file_util::file_exists(&*path) {
        // 如果文件存在，读取并反序列化
        let config_str = fs::read_to_string(path)?;

        let conf: Config = from_str(&config_str).unwrap_or_else(|_| {
            log::info!("配置文件存在，但格式不正确，使用默认配置修复...");
            Config::default()
        });
        log::info!("读取conf{:?}", conf);
        config = conf;
    } else {
        // 如果文件不存在，创建一个默认配置文件
        let default_config = Config::default();
        save_config(&default_config)?;
        config = default_config;
    }
    let config_clone = config.clone();
    let default_config = Config::default();
    // 修复配置文件
    let cache_path_merged = if config_clone.cache_path == None {
        default_config.cache_path
    } else {
        config_clone.cache_path
    };
    let image_cache_path_merged = if config_clone.image_cache_path == None {
        default_config.image_cache_path
    } else {
        config_clone.image_cache_path
    };
    let merged_config = Config {
        database_default_link: if config_clone.database_default_link == None {
            default_config.database_default_link
        } else {
            config_clone.database_default_link
        },
        database_name: if config_clone.database_name == None {
            default_config.database_name
        } else {
            config_clone.database_name
        },
        database_path: if config_clone.database_path == None {
            default_config.database_path
        } else {
            config_clone.database_path
        },
        cache_path: cache_path_merged.clone(),
        image_cache_path: image_cache_path_merged.clone(),
        thumbnail_storage_path: if config_clone.thumbnail_storage_path == None {
            // 获取 exe 文件路径
            let root_dir = get_root_folder().expect("根路径获取失败! ");
            let thumbnail_path = root_dir
                .join(cache_path_merged.unwrap())
                .join(image_cache_path_merged.unwrap());
            Some(thumbnail_path.display().to_string())
        } else {
            config_clone.thumbnail_storage_path
        },
        time_basic_fmt: if config_clone.time_basic_fmt == None {
            default_config.time_basic_fmt
        } else {
            config_clone.time_basic_fmt
        },
        extra: Default::default(),
    };
    // 如果配置有变动，保存修复后的配置
    if config != merged_config {
        log::info!("保存修复后的配置...");
        save_config(&merged_config)?;
        return Ok(merged_config)
    }
    Ok(config)
}

/// 公开的初始化
pub fn init_config() -> Config {
    SYS_CONFIG.clone()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 模拟从文件读取配置内容
    let config_str = r#"
    database_url = "postgres://user:password@localhost/dbname"
    log_path = "/var/log/app"
    "#;

    // 反序列化配置文件
    let config: Config = from_str(config_str)?;

    println!("{:?}", config);

    // 输出多余的字段
    for (key, value) in &config.extra {
        println!("Extra field - {}: {}", key, value);
    }

    Ok(())
}
