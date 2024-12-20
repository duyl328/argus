use crate::constant::DEFAULT_PROFILE_NAME;
use crate::utils::file_util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    database_url: String,
    #[serde(default)] // 如果配置文件中缺少该字段，使用默认值
    database_path: String,
    log_path: String,
    #[serde(default = "default_log_path")] // 自定义默认值
    log_level: String,
    #[serde(flatten)] // 收集多余的字段
    extra: HashMap<String, String>,
}

impl Config {
    pub fn update_database_url(&mut self, new_url: &str) {
        self.database_url = new_url.to_string();
    }

    pub fn add_extra_field(&mut self, key: String, value: String) {
        self.extra.insert(key, value);
    }

    pub fn default() -> Config {
        Config {
            database_url: "postgres://user:password@localhost/dbname".to_string(),
            database_path: "db".to_string(),
            log_path: "/var/log/app".to_string(),
            log_level: "".to_string(),
            extra: HashMap::new(),
        }
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = DEFAULT_PROFILE_NAME;
    let toml_string = to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = DEFAULT_PROFILE_NAME;
    // 检查配置文件是否存在
    if file_util::file_exists(path) {
        // 如果文件存在，读取并反序列化
        let config_str = fs::read_to_string(path)?;
        let config: Config = from_str(&config_str)?;
        Ok(config)
    } else {
        // 如果文件不存在，创建一个默认配置文件
        let default_config = Config::default();
        save_config(&default_config)?;
        Ok(default_config) // 返回默认配置
    }
}

fn default_log_path() -> String {
    "info".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 模拟从文件读取配置内容
    let config_str = r#"
    database_url = "postgres://user:password@localhost/dbname"
    log_path = "/var/log/app"
    "#;

    // 反序列化配置文件
    let config: Config = toml::de::from_str(config_str)?;

    println!("{:?}", config);

    // 输出多余的字段
    for (key, value) in &config.extra {
        println!("Extra field - {}: {}", key, value);
    }

    Ok(())
}
