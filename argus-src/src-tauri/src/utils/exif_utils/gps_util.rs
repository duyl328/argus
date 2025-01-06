use regex::Regex;

pub struct GpsUtil;

impl GpsUtil {
    /// 解析方向
    pub fn resolve_direction(direction: String) -> String {
        match direction.as_str() {
            "North" => String::from("N"),
            "East" => String::from("E"),
            _ => String::from(""),
        }
    }

    /// 解析具体坐标（度分秒）
    pub fn resolve_coordinate(coordinate: String) -> String {
        // 匹配度数、分度、秒度和方向
        let re = Regex::new(r"(\d+) deg (\d+)' (\d+\.\d+)").unwrap();
        let x: Vec<_> = coordinate
            .split("\"")
            .map(str::trim) // 去除每部分的前后空白
            .collect();

        // 进行替换
        let result = re.replace_all(&*x[0], |caps: &regex::Captures| {
            let degree = &caps[1];
            let minute = &caps[2];
            let second = &caps[3];

            format!("{}°{}′{}″", degree, minute, second)
        });

        result.to_string()
    }

    /// 解析海拔
    pub fn resolve_altitude(coordinate: String) -> String {
        coordinate.replace(" m ", "m").replace("Above Sea Level", "")
    }
}

mod tests {
    use crate::utils::exif_utils::gps_util::GpsUtil;

    #[test]
    fn test1() {
        let str = "114 deg 9' 56.09\" E";
        let string = GpsUtil::resolve_coordinate(str.to_string());
        println!("{}", string)
    }

    #[test]
    fn test2() {
        let str = "6 m Above Sea Level";
        let string = GpsUtil::resolve_altitude(str.to_string());
        println!("{}", string)
    }
}
