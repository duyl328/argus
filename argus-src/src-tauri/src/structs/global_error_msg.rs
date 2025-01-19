use crate::utils::img_util::ImageOperate;
use once_cell::sync::Lazy;
use serde;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Window};
use tokio::sync::mpsc::Sender;

/// emit 状态是否初始化
pub static GLOBAL_EMIT_IS_INIT: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
/// 全局触发实例
pub static GLOBAL_EMIT_APP_HANDLE: Lazy<Arc<Mutex<Option<Sender<String>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None::<Sender<String>>)));

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalErrorMsg {
    /// 标题
    pub title: String,

    /// 展示消息
    pub msg: String,

    /// 展示时间【为 0 时则不会自动关闭】
    pub duration: i32,

    /// 通知类型
    #[serde(rename = "type")] // 指定序列化/反序列化时的字段名为 "type"
    pub kind: GlobalErrorMsgTypeEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GlobalErrorMsgTypeEnum {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "")]
    None,
}
impl PartialEq for GlobalErrorMsgTypeEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GlobalErrorMsgTypeEnum::Success, GlobalErrorMsgTypeEnum::Success) => true,
            (GlobalErrorMsgTypeEnum::Warning, GlobalErrorMsgTypeEnum::Warning) => true,
            (GlobalErrorMsgTypeEnum::Info, GlobalErrorMsgTypeEnum::Info) => true,
            (GlobalErrorMsgTypeEnum::Error, GlobalErrorMsgTypeEnum::Error) => true,
            (GlobalErrorMsgTypeEnum::None, GlobalErrorMsgTypeEnum::None) => true,
            _ => false,
        }
    }
}
impl GlobalErrorMsgTypeEnum {
    // 为枚举定义一个方法，返回对应的字符串
    fn as_str(&self) -> &'static str {
        match self {
            GlobalErrorMsgTypeEnum::Success => "success",
            GlobalErrorMsgTypeEnum::Warning => "warning",
            GlobalErrorMsgTypeEnum::Info => "info",
            GlobalErrorMsgTypeEnum::Error => "error",
            GlobalErrorMsgTypeEnum::None => "",
        }
    }

    // 一个示例方法，用于从字符串创建 Direction 枚举
    fn from_str(input: &str) -> Option<GlobalErrorMsgTypeEnum> {
        match input {
            "success" => Some(GlobalErrorMsgTypeEnum::Success),
            "warning" => Some(GlobalErrorMsgTypeEnum::Warning),
            "info" => Some(GlobalErrorMsgTypeEnum::Info),
            "error" => Some(GlobalErrorMsgTypeEnum::Error),
            _ => None, // 返回 None 表示无效输入
        }
    }
}

#[cfg(test)]
mod test {
    use crate::structs::global_error_msg::GlobalErrorMsg;
    use crate::structs::global_error_msg::GlobalErrorMsgTypeEnum;
    use crate::utils::json_util::JsonUtil;
    use std::cmp::PartialEq;

    #[test]
    fn to_json() {
        let gem = GlobalErrorMsg {
            title: "标题".parse().unwrap(),
            msg: "信息".to_string(),
            duration: 0,
            kind: GlobalErrorMsgTypeEnum::Success,
        };
        let result = JsonUtil::stringify(&gem).expect("序列化报错");

        let result1: GlobalErrorMsg = JsonUtil::from_json(&result).expect("反序列化出错");

        assert_eq!(gem.kind, result1.kind); // 断言结果为 5
    }
}
