use serde::{Deserialize, Serialize};
use serde_json::{self, Value, Error};
use crate::errors::JsonError;

pub struct JsonUtil;
impl JsonUtil {
    /// 通用函数：序列化
    pub fn stringify<T: Serialize>(value: &T) -> Result<String, JsonError> {
        let result = serde_json::to_string(value);
        if result.is_ok() {
            Ok(result?)
        } else {
            Err(JsonError::Error(result.unwrap_err()))
        }
    }

    /// 通用函数：反序列化
    pub fn from_json<'a, T: Deserialize<'a>>(json_str: &'a str) -> Result<T, Error> {
        serde_json::from_str(json_str)
    }
    
    /// 获取单层键值
    pub fn get_value(json_str: &str, key: &str) -> Option<Value> {
        match serde_json::from_str::<Value>(json_str) {
            Ok(parsed) => parsed.get(key).cloned(),
            Err(err) => {
                log::error!("Failed to parse JSON: {}", err);
                None
            }
        }
    }

    
    pub fn set_value(json_str: &str, key: &str, value: Value) -> Option<String> {
        let mut parsed: Value = serde_json::from_str(json_str).ok()?;
        if let Value::Object(ref mut map) = parsed {
            map.insert(key.to_string(), value);
            return serde_json::to_string(&parsed).ok();
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use serde_json::Error;
    use crate::utils::json_util::JsonUtil;

    #[test]
    fn test_parse_arg() {
        let vec = vec!["arg1", "arg2"];
        let mut x:HashSet < String > = HashSet::new();
        x.insert("1".to_string());
        x.insert("2".to_string());
        x.insert("3".to_string());
        let result = JsonUtil::stringify(&x);
        log::info!("ans data: {:?}", result);
        assert!(result.is_ok());
        let string = result.unwrap();

        assert_eq!(string, "[\"arg1\",\"arg2\"]");
    }

    #[test]
    fn test_parse_empty_array() {
        let str = "[\"arg1\",\"arg2\"]";
        let result:Result<Vec<String>,Error> = JsonUtil::from_json(str);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
    }
}