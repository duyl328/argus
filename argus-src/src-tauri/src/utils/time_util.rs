extern crate chrono;
use crate::constant::TIME_BASIC_FMT;
pub use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前的时间戳（秒）
    pub fn current_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    /// 获取当前的时间戳（毫秒）
    pub fn current_timestamp_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    /// 将时间戳转换为 `NaiveDateTime`
    pub fn timestamp_to_naive_date_time(timestamp: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(timestamp, 0).expect("invalid timestamp")
    }

    /// 将时间戳（秒）转换为 `String` 格式的日期
    pub fn timestamp_to_string(timestamp: i64, fmt: Option<&str>) -> String {
        let naive = TimeUtils::timestamp_to_naive_date_time(timestamp);
        let fmt = fmt.unwrap_or(TIME_BASIC_FMT);
        naive.format(fmt).to_string()
    }

    /// 获取当前时间的 `String` 格式的日期
    pub fn current_datetime_string(fmt: Option<&str>) -> String {
        let fmt = fmt.unwrap_or(TIME_BASIC_FMT);
        Utc::now().format(fmt).to_string()
    }

    /// 将格式化日期字符串转换为 `NaiveDateTime`
    pub fn string_to_naive_date_time(date_str: &str, fmt: Option<&str>) -> Option<NaiveDateTime> {
        let fmt = fmt.unwrap_or(TIME_BASIC_FMT);
        NaiveDateTime::parse_from_str(date_str, fmt).ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    const TIME_BASIC_FMT: &str = "%Y-%m-%d %H:%M:%S"; // 用于日期格式化的默认格式

    #[test]
    fn test_current_timestamp() {
        let timestamp = TimeUtils::current_timestamp();
        let current_timestamp = Utc::now().timestamp();
        // 允许小的误差（例如，函数执行时的时间差）
        assert!(timestamp >= current_timestamp - 1 && timestamp <= current_timestamp + 1);
    }

    #[test]
    fn test_current_timestamp_millis() {
        let timestamp_millis = TimeUtils::current_timestamp_millis();
        let current_timestamp_millis = Utc::now().timestamp_millis();
        // 允许小的误差（例如，函数执行时的时间差）
        assert!(timestamp_millis >= current_timestamp_millis - 1 && timestamp_millis <= current_timestamp_millis + 1);
    }

    #[test]
    fn test_timestamp_to_naive_date_time() {
        let timestamp = Utc::now().timestamp();
        let naive_datetime = TimeUtils::timestamp_to_naive_date_time(timestamp);
        // 确保返回的 `DateTime<Utc>` 可以转换为 `NaiveDateTime` 类型
        assert_eq!(naive_datetime.timestamp(), timestamp);
    }

    #[test]
    fn test_timestamp_to_string() {
        let timestamp = Utc::now().timestamp();
        let date_str = TimeUtils::timestamp_to_string(timestamp, Some(TIME_BASIC_FMT));
        // 使用自定义格式验证返回值
        assert!(date_str.contains("-") && date_str.contains(":"));
    }

    #[test]
    fn test_current_datetime_string() {
        let date_str = TimeUtils::current_datetime_string(Some(TIME_BASIC_FMT));
        // 验证当前时间的字符串是否符合预期格式
        assert!(date_str.contains("-") && date_str.contains(":"));
    }

    #[test]
    fn test_string_to_naive_date_time() {
        let date_str = "2024-12-10 12:34:56";
        let naive_datetime = TimeUtils::string_to_naive_date_time(date_str, Some(TIME_BASIC_FMT));
        // 确保日期解析成功
        assert!(naive_datetime.is_some());
        assert_eq!(naive_datetime.unwrap().format(TIME_BASIC_FMT).to_string(), date_str);
    }

    #[test]
    fn test_string_to_naive_date_time_invalid() {
        let date_str = "invalid-date";
        let naive_datetime = TimeUtils::string_to_naive_date_time(date_str, Some(TIME_BASIC_FMT));
        // 验证无效的日期字符串返回 None
        assert!(naive_datetime.is_none());
    }
}
