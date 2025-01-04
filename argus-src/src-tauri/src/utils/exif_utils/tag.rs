use crate::utils::exif_utils::value::ValueType;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tag {
    /// 原始数据保存
    pub entries: Vec<(String, String)>,
    /// 数据映射
    pub entry_map: HashMap<String, String>,
}

impl Tag {
    pub fn parse(mut self, info: &str) -> Self {
        for line in info.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                self.entry_map.insert(key.clone(), value.clone());
                self.entries.push((key, value));
            }
        }
        self
    }

    pub fn new() -> Self {
        Tag {
            entries: Vec::new(),
            entry_map: HashMap::new(),
        }
    }
}

pub struct ExifToolDesc {}

impl ExifToolDesc {
    pub const MAKE: ExifInfo = ExifInfo {
        dis: "相机制造商",
        exif_tool_desc: "Make",
        value_type: ValueType::String,
    };
    pub const MODEL: ExifInfo = ExifInfo {
        dis: "相机型号",
        exif_tool_desc: "Camera Model Name",
        value_type: ValueType::String,
    };
    pub const SOFTWARE: ExifInfo = ExifInfo {
        dis: "软件",
        exif_tool_desc: "Software",
        value_type: ValueType::String,
    };
    pub const EXPOSURE_TIME: ExifInfo = ExifInfo {
        dis: "快门速度",
        exif_tool_desc: "Exposure Time",
        value_type: ValueType::String,
    };
    pub const F_NUMBER: ExifInfo = ExifInfo {
        dis: "光圈数",
        exif_tool_desc: "F Number",
        value_type: ValueType::String,
    };
    pub const ISO: ExifInfo = ExifInfo {
        dis: "ISO 感光度",
        exif_tool_desc: "ISO",
        value_type: ValueType::String,
    };
    pub const EXIF_VERSION: ExifInfo = ExifInfo {
        dis: "Exif 版本",
        exif_tool_desc: "Exif Version",
        value_type: ValueType::String,
    };
    pub const DATE_TIME_ORIGINAL: ExifInfo = ExifInfo {
        dis: "拍摄时间",
        exif_tool_desc: "Date/Time Original",
        value_type: ValueType::String,
    };
    pub const OFFSET_TIME: ExifInfo = ExifInfo {
        dis: "时区",
        exif_tool_desc: "Offset Time",
        value_type: ValueType::String,
    };
    pub const MAX_APERTURE_VALUE: ExifInfo = ExifInfo {
        dis: "最大光圈",
        exif_tool_desc: "Max Aperture Value",
        value_type: ValueType::String,
    };
    pub const FOCAL_LENGTH: ExifInfo = ExifInfo {
        dis: "焦距",
        exif_tool_desc: "Focal Length",
        value_type: ValueType::String,
    };
    pub const FOCAL_LENGTH_IN_35MM_FORMAT: ExifInfo = ExifInfo {
        dis: "等效焦距",
        exif_tool_desc: "Focal Length In 35mm Format",
        value_type: ValueType::String,
    };
    pub const IMAGE_WIDTH: ExifInfo = ExifInfo {
        dis: "图像宽度",
        exif_tool_desc: "Image Width",
        value_type: ValueType::String,
    };
    pub const IMAGE_HEIGHT: ExifInfo = ExifInfo {
        dis: "图像长度",
        exif_tool_desc: "Image Height",
        value_type: ValueType::String,
    };
    pub const GPS_LATITUDE_REF: ExifInfo = ExifInfo {
        dis: "GPS 纬度参考",
        exif_tool_desc: "GPS Latitude Ref",
        value_type: ValueType::String,
    };
    pub const GPS_LONGITUDE_REF: ExifInfo = ExifInfo {
        dis: "GPS 经度参考",
        exif_tool_desc: "GPS Longitude Ref",
        value_type: ValueType::String,
    };
    pub const GPS_LATITUDE: ExifInfo = ExifInfo {
        dis: "GPS 纬度",
        exif_tool_desc: "GPS Latitude",
        value_type: ValueType::String,
    };
    pub const GPS_LONGITUDE: ExifInfo = ExifInfo {
        dis: "GPS 经度",
        exif_tool_desc: "GPS Longitude",
        value_type: ValueType::String,
    };
    pub const GPS_ALTITUDE: ExifInfo = ExifInfo {
        dis: "GPS 海拔",
        exif_tool_desc: "GPS Altitude",
        value_type: ValueType::String,
    };
    pub const EXPOSURE_PROGRAM: ExifInfo = ExifInfo {
        dis: "曝光程序",
        exif_tool_desc: "Exposure Program",
        value_type: ValueType::String,
    };
    pub const METERING_MODE: ExifInfo = ExifInfo {
        dis: "测光模式",
        exif_tool_desc: "Metering Mode",
        value_type: ValueType::String,
    };
    pub const FLASH: ExifInfo = ExifInfo {
        dis: "闪光灯",
        exif_tool_desc: "Flash",
        value_type: ValueType::String,
    };
}

#[derive(Clone, Debug)]
pub struct ExifInfo {
    pub dis: &'static str,
    /// exiftool 的文字描述（匹配数据）
    pub exif_tool_desc: &'static str,
    /// 数据类型
    pub value_type: ValueType,
}

// todo: 2025/1/2 21:05 该部分目前只关心 gps ，暂不拓展
/*
现在的目的是实现结构体的定义和现有数据的匹配，数据的解析等功能暂不考虑
*/
// 名称、描述、类型、默认值
// 生成标签常量
// macro_rules! generate_tag_constants {
//     (
//         $(
//             // 捕获传递的属性【可空】
//             // $(#[$attr:meta])*
//             ($name:ident ,$value:expr,$defval:expr, $desc:expr,$ExifToolDesc:expr, $type:ty)
//         )+
//
//     ) => (
//         struct ExifTag {
//             pub const $name: Tag = Tag($ctx, $num);
//         }
//
//     );
// }
//
// generate_tag_constants!();
