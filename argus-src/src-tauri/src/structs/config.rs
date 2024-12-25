use crate::conf;
use crate::conf::{Conf, CONF_DEFAULT};
use crate::constant::DEFAULT_PROFILE_NAME;
use crate::utils::file_util;
use crate::utils::file_util::{create_folder, get_root_folder};
use crate::utils::json_util::JsonUtil;
use anyhow::Result;
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
    /// 目录界别
    pub directory_level: Option<u32>,

    // Python 相关配置
    /// Python 服务地址
    pub python_service_path: Option<String>,

    #[serde(flatten)] // 收集多余的字段
    extra: HashMap<String, String>,
}

impl Config {
    pub fn add_extra_field(&mut self, key: String, value: String) {
        self.extra.insert(key, value);
    }

    pub fn default() -> Config {
        Config {
            database_default_link: Some(CONF_DEFAULT.database_default_link.clone()),
            database_name: Some(CONF_DEFAULT.database_name.clone()),
            database_path: Some(CONF_DEFAULT.database_path.clone()),
            image_cache_path: Some(CONF_DEFAULT.image_cache_path.clone()),
            cache_path: Some(CONF_DEFAULT.cache_path.clone()),
            thumbnail_storage_path: None,
            time_basic_fmt: Some(CONF_DEFAULT.time_basic_fmt.clone()),
            directory_level: Some(CONF_DEFAULT.directory_level.clone()),
            python_service_path: Some(CONF_DEFAULT.python_service_path.clone()),
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
            && self.directory_level == other.directory_level
            && self.python_service_path == other.python_service_path
            && self.extra == other.extra
    }
}

pub static SYS_CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = load_config().expect("系统配置文件加载失败! ");
    // 构建必须参数
    config
});

/// 获取配置文件存放路径
fn get_config_dir() -> String {
    let root_dir = get_root_folder().expect("根路径获取失败! ");
    root_dir.join(DEFAULT_PROFILE_NAME).display().to_string()
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_dir();
    let toml_string = to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

fn load_config() -> Result<Config> {
    log::info!("load_config");
    let path = get_config_dir();
    log::info!("餐速回构建完毕!!!!!!!!!!!!!  ");
    let str = JsonUtil::stringify(&path)?;
    log::info!("{}  ", str);
    let mut data = conf::CONF.write().expect("write 报错了");
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
    // 修复配置文件
    let cache_path_merged = config_clone
        .cache_path
        .unwrap_or_else(|| data.cache_path.clone());
    let image_cache_path_merged = config_clone
        .image_cache_path
        .unwrap_or_else(|| data.image_cache_path.clone());
    // 缩略图保存路径
    let thumbnail_storage_path_merged = if config_clone.thumbnail_storage_path == None {
        // 获取 exe 文件路径
        let root_dir = get_root_folder()?;
        let thumbnail_path = root_dir
            .join(cache_path_merged.clone())
            .join(image_cache_path_merged.clone());
        let string = thumbnail_path.display().to_string();
        data.thumbnail_storage_path = string.clone();
        Some(string)
    } else {
        config_clone.thumbnail_storage_path
    };
    let merged_config = Config {
        database_default_link: Some(
            config_clone
                .database_default_link
                .unwrap_or_else(|| data.database_default_link.clone()),
        ),
        database_name: Some(
            config_clone
                .database_name
                .unwrap_or_else(|| data.database_name.clone()),
        ),
        database_path: Some(
            config_clone
                .database_path
                .unwrap_or_else(|| data.database_path.clone()),
        ),
        cache_path: Some(cache_path_merged.clone()),
        image_cache_path: Some(image_cache_path_merged.clone()),
        thumbnail_storage_path: thumbnail_storage_path_merged,
        time_basic_fmt: Some(
            config_clone
                .time_basic_fmt
                .unwrap_or_else(|| data.time_basic_fmt.clone()),
        ),
        directory_level: Some(
            config_clone
                .directory_level
                .unwrap_or_else(|| data.directory_level.clone()),
        ),
        python_service_path: Some(
            config_clone
                .python_service_path
                .unwrap_or_else(|| data.python_service_path.clone()),
        ),
        extra: Default::default(),
    };
    // 如果配置有变动，保存修复后的配置
    if config != merged_config {
        log::info!("保存修复后的配置...");
        save_config(&merged_config)?;
        return Ok(merged_config);
    }
    Ok(config)
}

/// 公开的初始化
pub fn init_config() -> Config {
    log::info!("进入初始化");
    SYS_CONFIG.clone()
}
